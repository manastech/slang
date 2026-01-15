use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use anyhow::Result;
use slang_solidity::backend::semantic::ast::Definition;
use slang_solidity::backend::semantic::{ast, SemanticAnalysis};
use slang_solidity::cst::NodeId;

use super::collect_definitions::CollectDefinitionsVisitor;
use crate::backend::fixtures;

fn find_unused_definitions_from_contract_name(
    semantic: &Rc<SemanticAnalysis>,
    starting_contract: &str,
) -> Vec<Definition> {
    let mut unused_definitions: HashMap<NodeId, Definition> = HashMap::new();
    let mut visit_queue = VecDeque::new();

    unused_definitions.extend(
        semantic
            .all_definitions()
            .map(|definition| (definition.node_id(), definition)),
    );
    let Some(contract) = semantic.find_contract_by_name(starting_contract) else {
        return Vec::new();
    };
    visit_queue.push_back(contract.as_definition());

    while let Some(to_visit) = visit_queue.pop_front() {
        let node_id = to_visit.node_id();
        if !unused_definitions.contains_key(&node_id) {
            continue;
        }
        unused_definitions.remove(&node_id);

        let definitions = gather_referenced_definitions_from(&to_visit);
        for followed in definitions {
            visit_queue.push_back(followed);
        }
    }

    // For remaining unused definitions we want to remove any nested definitions
    // inside them to prevent reporting eg. a function and all its parameters.
    visit_queue.extend(unused_definitions.values().cloned());
    while let Some(to_visit) = visit_queue.pop_front() {
        let to_visit_id = to_visit.node_id();
        if !unused_definitions.contains_key(&to_visit_id) {
            continue;
        }

        // TODO: for now we explicitly remove the imported symbols because any
        // references in the file would skip them for the actual definition, but
        // we want to track their usage properly
        if matches!(to_visit, Definition::ImportedSymbol(_)) {
            unused_definitions.remove(&to_visit_id);
            continue;
        }

        let mut visitor = CollectDefinitionsVisitor::default();
        accept_definition(&to_visit, &mut visitor);
        for inner_definition in visitor.finish() {
            let inner_id = inner_definition.node_id();
            if inner_id == to_visit_id {
                continue;
            }
            unused_definitions.remove(&inner_id);
        }
    }

    unused_definitions.into_values().collect()
}

fn gather_referenced_definitions_from(definition: &Definition) -> Vec<Definition> {
    match definition {
        Definition::Contract(contract_definition) => {
            // for contracts, everything public should be automatically referenced
            gather_referenced_definitions_from_contract(contract_definition)
        }
        Definition::Struct(_) | Definition::Library(_) | Definition::Enum(_) => {
            // members inside these need to be explicitly referenced
            Vec::new()
        }
        Definition::Function(function_definition) | Definition::Modifier(function_definition) => {
            // follow and return any references in function and modifier bodies/signatures
            let mut visitor = FollowReferencesVisitor::default();
            ast::visitor::accept_function_definition(function_definition, &mut visitor);
            visitor.finish()
        }
        _ => {
            // Anything else (events, errors, interfaces) should be considered fully
            // referenced (including inner definitions) and we need to (later) follow any
            // references inside them.
            let mut visitor = CollectDefinitionsVisitor::default();
            accept_definition(definition, &mut visitor);
            visitor.finish()
        }
    }
}

fn gather_referenced_definitions_from_contract(
    contract: &ast::ContractDefinition,
) -> Vec<Definition> {
    // To gather referenced definitions in a contract, we need to consider
    // public functions, constructors and state variables and any references in
    // the inheritance list or storage layout specifiers.
    let mut visitor = FollowReferencesVisitor::default();
    ast::visitor::accept_inheritance_types(&contract.inheritance_types(), &mut visitor);
    if let Some(storage_layout) = contract.storage_layout() {
        ast::visitor::accept_expression(&storage_layout, &mut visitor);
    }
    for member in contract.members().iter() {
        match member {
            ast::ContractMember::FunctionDefinition(function_definition) => {
                if matches!(
                    function_definition.kind(),
                    ast::FunctionKind::Constructor
                        | ast::FunctionKind::Fallback
                        | ast::FunctionKind::Receive
                        | ast::FunctionKind::Unnamed
                ) {
                    ast::visitor::accept_function_definition(&function_definition, &mut visitor);
                } else if function_definition.name().is_some()
                    && matches!(
                        function_definition.visibility(),
                        ast::FunctionVisibility::External | ast::FunctionVisibility::Public
                    )
                {
                    visitor.add_definition(function_definition.as_definition());
                }
            }
            ast::ContractMember::StateVariableDefinition(state_variable_definition) => {
                if matches!(
                    state_variable_definition.visibility(),
                    ast::StateVariableVisibility::Public
                ) {
                    visitor.add_definition(state_variable_definition.as_definition());
                }
            }
            _ => {}
        }
    }
    visitor.finish()
}

// AST visitor that collects all definitions pointed by references
#[derive(Default)]
pub(crate) struct FollowReferencesVisitor {
    definitions: Vec<Definition>,
}

impl FollowReferencesVisitor {
    pub(crate) fn finish(self) -> Vec<Definition> {
        self.definitions
    }
    pub(crate) fn add_definition(&mut self, definition: Definition) {
        self.definitions.push(definition);
    }
}

impl ast::visitor::Visitor for FollowReferencesVisitor {
    fn visit_identifier(&mut self, identifier: &ast::Identifier) {
        if let Some(definition) = identifier.resolve_to_definition() {
            self.add_definition(definition);
        }
    }
}

fn accept_definition(definition: &Definition, visitor: &mut impl ast::visitor::Visitor) {
    match definition {
        Definition::Constant(constant_definition) => {
            ast::visitor::accept_constant_definition(constant_definition, visitor);
        }
        Definition::Contract(contract_definition) => {
            ast::visitor::accept_contract_definition(contract_definition, visitor);
        }
        Definition::Enum(enum_definition) => {
            ast::visitor::accept_enum_definition(enum_definition, visitor);
        }
        Definition::Error(error_definition) => {
            ast::visitor::accept_error_definition(error_definition, visitor);
        }
        Definition::Event(event_definition) => {
            ast::visitor::accept_event_definition(event_definition, visitor);
        }
        Definition::Function(function_definition) | Definition::Modifier(function_definition) => {
            ast::visitor::accept_function_definition(function_definition, visitor);
        }
        Definition::Library(library_definition) => {
            ast::visitor::accept_library_definition(library_definition, visitor);
        }
        Definition::Import(path_import) => {
            ast::visitor::accept_path_import(path_import, visitor);
        }
        Definition::ImportedSymbol(import_deconstruction_symbol) => {
            ast::visitor::accept_import_deconstruction_symbol(
                import_deconstruction_symbol,
                visitor,
            );
        }
        Definition::Interface(interface_definition) => {
            ast::visitor::accept_interface_definition(interface_definition, visitor);
        }
        Definition::Parameter(parameter) => {
            ast::visitor::accept_parameter(parameter, visitor);
        }
        Definition::StateVariable(state_variable_definition) => {
            ast::visitor::accept_state_variable_definition(state_variable_definition, visitor);
        }
        Definition::Struct(struct_definition) => {
            ast::visitor::accept_struct_definition(struct_definition, visitor);
        }
        Definition::StructMember(struct_member) => {
            ast::visitor::accept_struct_member(struct_member, visitor);
        }
        Definition::TypeParameter(parameter) => {
            ast::visitor::accept_parameter(parameter, visitor);
        }
        Definition::UserDefinedValueType(user_defined_value_type_definition) => {
            ast::visitor::accept_user_defined_value_type_definition(
                user_defined_value_type_definition,
                visitor,
            );
        }
        Definition::Variable(variable_declaration_statement) => {
            ast::visitor::accept_variable_declaration_statement(
                variable_declaration_statement,
                visitor,
            );
        }
        Definition::YulFunction(yul_function_definition) => {
            ast::visitor::accept_yul_function_definition(yul_function_definition, visitor);
        }
        Definition::YulLabel(yul_label) => {
            ast::visitor::accept_yul_label(yul_label, visitor);
        }
        Definition::EnumMember(identifier)
        | Definition::YulParameter(identifier)
        | Definition::YulVariable(identifier) => {
            ast::visitor::accept_identifier(identifier, visitor);
        }
    }
}

#[test]
fn test_no_unused_definitions() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let unused_definitions = find_unused_definitions_from_contract_name(semantic, "Counter");
    assert_eq!(unused_definitions.len(), 0);

    Ok(())
}

#[test]
fn test_some_unused_member_definitions() -> Result<()> {
    let unit = fixtures::UnusedMembers::build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let mut unused_definitions = find_unused_definitions_from_contract_name(semantic, "Counter");
    unused_definitions.sort_by_key(|definition| definition.identifier().unparse());

    assert_eq!(unused_definitions.len(), 4);
    assert_eq!(unused_definitions[0].identifier().unparse(), "_unused");
    assert_eq!(unused_definitions[1].identifier().unparse(), "checkOwner");
    assert_eq!(unused_definitions[2].identifier().unparse(), "multiplier");
    assert_eq!(
        unused_definitions[3].identifier().unparse(),
        "unusedDecrement"
    );

    Ok(())
}
