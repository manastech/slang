use anyhow::Result;
use paste::paste;
use slang_solidity::backend::semantic::ast::{self, visitor, Definition};

use crate::backend::fixtures;

#[derive(Default)]
pub(crate) struct CollectDefinitionsVisitor {
    definitions: Vec<Definition>,
}

impl CollectDefinitionsVisitor {
    pub(crate) fn finish(self) -> Vec<Definition> {
        self.definitions
    }
}

macro_rules! define_enter {
    ($type:ident, $value:expr) => {
        paste! {
            fn [<enter_ $type:snake>](&mut self, node: &ast::$type) -> bool {
                self.definitions.push(node.as_definition());
                $value
            }
        }
    };
}

impl visitor::Visitor for CollectDefinitionsVisitor {
    define_enter!(ConstantDefinition, false);
    define_enter!(ContractDefinition, true);
    define_enter!(EnumDefinition, true);
    define_enter!(ErrorDefinition, true);
    define_enter!(EventDefinition, true);
    define_enter!(ImportDeconstructionSymbol, false);
    define_enter!(InterfaceDefinition, true);
    define_enter!(LibraryDefinition, true);
    define_enter!(PathImport, false);
    define_enter!(StateVariableDefinition, false);
    define_enter!(StructDefinition, true);
    define_enter!(StructMember, false);
    define_enter!(UserDefinedValueTypeDefinition, false);
    define_enter!(VariableDeclarationStatement, false);
    define_enter!(YulLabel, false);
    define_enter!(YulFunctionDefinition, true);

    // functions are a special case because they can be unnamed (eg.
    // constructors) and they don't generate a definition
    fn enter_function_definition(&mut self, node: &ast::FunctionDefinition) -> bool {
        if node.name().is_some() {
            self.definitions.push(node.as_definition());
        }
        true
    }

    // ditto with parameters
    fn enter_parameter(&mut self, node: &ast::Parameter) -> bool {
        if node.name().is_some() {
            self.definitions.push(node.as_definition());
        }
        true
    }

    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if node.is_definition() {
            self.definitions.push(node.as_definition());
        }
    }

    fn visit_yul_identifier(&mut self, node: &ast::YulIdentifier) {
        if node.is_definition() {
            self.definitions.push(node.as_definition());
        }
    }
}

#[test]
fn test_collect_definitions() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
        .find_contract_by_name("Counter")
        .expect("contract is found");

    let mut visitor = CollectDefinitionsVisitor::default();
    visitor::accept_contract_definition(&counter, &mut visitor);

    let found_definitions = visitor.finish();
    assert_eq!(found_definitions.len(), 7);
    assert_eq!(found_definitions[0].identifier().unparse(), "Counter");
    assert_eq!(found_definitions[1].identifier().unparse(), "count");
    assert_eq!(found_definitions[2].identifier().unparse(), "_clickers");
    assert_eq!(found_definitions[3].identifier().unparse(), "initial");
    assert_eq!(found_definitions[4].identifier().unparse(), "increment");
    assert_eq!(found_definitions[5].identifier().unparse(), "delta");
    assert_eq!(found_definitions[6].identifier().unparse(), "click");

    Ok(())
}
