// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;
use std::vec::Vec;

use metaslang_cst::nodes::NodeId;

use crate::cst::{SyntaxNode, TerminalNode};

//
// Sequences:
//

pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug)]
pub struct SourceUnitStruct {
    pub node: Rc<SyntaxNode>,
    pub members: SourceUnitMembers,
}

impl SourceUnitStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Debug)]
pub struct PragmaDirectiveStruct {
    pub node: Rc<SyntaxNode>,
    pub pragma: Pragma,
}

impl PragmaDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Debug)]
pub struct AbicoderPragmaStruct {
    pub node: Rc<SyntaxNode>,
    pub version: AbicoderVersion,
}

impl AbicoderPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Debug)]
pub struct ExperimentalPragmaStruct {
    pub node: Rc<SyntaxNode>,
    pub feature: ExperimentalFeature,
}

impl ExperimentalPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Debug)]
pub struct VersionPragmaStruct {
    pub node: Rc<SyntaxNode>,
    pub sets: VersionExpressionSets,
}

impl VersionPragmaStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Debug)]
pub struct VersionRangeStruct {
    pub node: Rc<SyntaxNode>,
    pub start: VersionLiteral,
    pub end: VersionLiteral,
}

impl VersionRangeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Debug)]
pub struct VersionTermStruct {
    pub node: Rc<SyntaxNode>,
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

impl VersionTermStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Debug)]
pub struct ImportDirectiveStruct {
    pub node: Rc<SyntaxNode>,
    pub clause: ImportClause,
}

impl ImportDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Debug)]
pub struct PathImportStruct {
    pub node: Rc<SyntaxNode>,
    pub path: StringLiteral,
    pub alias: Option<ImportAlias>,
}

impl PathImportStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Debug)]
pub struct NamedImportStruct {
    pub node: Rc<SyntaxNode>,
    pub alias: ImportAlias,
    pub path: StringLiteral,
}

impl NamedImportStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionStruct {
    pub node: Rc<SyntaxNode>,
    pub symbols: ImportDeconstructionSymbols,
    pub path: StringLiteral,
}

impl ImportDeconstructionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct ImportDeconstructionSymbolStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub alias: Option<ImportAlias>,
}

impl ImportDeconstructionSymbolStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ImportAlias = Rc<ImportAliasStruct>;

#[derive(Debug)]
pub struct ImportAliasStruct {
    pub node: Rc<SyntaxNode>,
    pub identifier: Rc<TerminalNode>,
}

impl ImportAliasStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Debug)]
pub struct UsingDirectiveStruct {
    pub node: Rc<SyntaxNode>,
    pub clause: UsingClause,
    pub target: UsingTarget,
    pub global_keyword: Option<Rc<TerminalNode>>,
}

impl UsingDirectiveStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionStruct {
    pub node: Rc<SyntaxNode>,
    pub symbols: UsingDeconstructionSymbols,
}

impl UsingDeconstructionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Debug)]
pub struct UsingDeconstructionSymbolStruct {
    pub node: Rc<SyntaxNode>,
    pub name: IdentifierPath,
    pub alias: Option<UsingAlias>,
}

impl UsingDeconstructionSymbolStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UsingAlias = Rc<UsingAliasStruct>;

#[derive(Debug)]
pub struct UsingAliasStruct {
    pub node: Rc<SyntaxNode>,
    pub operator: UsingOperator,
}

impl UsingAliasStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Debug)]
pub struct ContractDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub abstract_keyword: Option<Rc<TerminalNode>>,
    pub name: Rc<TerminalNode>,
    pub specifiers: ContractSpecifiers,
    pub members: ContractMembers,
}

impl ContractDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InheritanceSpecifier = Rc<InheritanceSpecifierStruct>;

#[derive(Debug)]
pub struct InheritanceSpecifierStruct {
    pub node: Rc<SyntaxNode>,
    pub types: InheritanceTypes,
}

impl InheritanceSpecifierStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Debug)]
pub struct InheritanceTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

impl InheritanceTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StorageLayoutSpecifier = Rc<StorageLayoutSpecifierStruct>;

#[derive(Debug)]
pub struct StorageLayoutSpecifierStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Expression,
}

impl StorageLayoutSpecifierStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Debug)]
pub struct InterfaceDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub inheritance: Option<InheritanceSpecifier>,
    pub members: InterfaceMembers,
}

impl InterfaceDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Debug)]
pub struct LibraryDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub members: LibraryMembers,
}

impl LibraryDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Debug)]
pub struct StructDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub members: StructMembers,
}

impl StructDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StructMember = Rc<StructMemberStruct>;

#[derive(Debug)]
pub struct StructMemberStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
}

impl StructMemberStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Debug)]
pub struct EnumDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub members: EnumMembers,
}

impl EnumDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Debug)]
pub struct ConstantDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Rc<TerminalNode>,
    pub value: Expression,
}

impl ConstantDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub attributes: StateVariableAttributes,
    pub name: Rc<TerminalNode>,
    pub value: Option<StateVariableDefinitionValue>,
}

impl StateVariableDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type StateVariableDefinitionValue = Rc<StateVariableDefinitionValueStruct>;

#[derive(Debug)]
pub struct StateVariableDefinitionValueStruct {
    pub node: Rc<SyntaxNode>,
    pub value: Expression,
}

impl StateVariableDefinitionValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: FunctionName,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

impl FunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ParametersDeclaration = Rc<ParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ParametersDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: Parameters,
}

impl ParametersDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Debug)]
pub struct ParameterStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Rc<TerminalNode>>,
}

impl ParameterStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Debug)]
pub struct OverrideSpecifierStruct {
    pub node: Rc<SyntaxNode>,
    pub overridden: Option<OverridePathsDeclaration>,
}

impl OverrideSpecifierStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type OverridePathsDeclaration = Rc<OverridePathsDeclarationStruct>;

#[derive(Debug)]
pub struct OverridePathsDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub paths: OverridePaths,
}

impl OverridePathsDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ReturnsDeclaration = Rc<ReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct ReturnsDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub variables: ParametersDeclaration,
}

impl ReturnsDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ConstructorDefinition = Rc<ConstructorDefinitionStruct>;

#[derive(Debug)]
pub struct ConstructorDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: ConstructorAttributes,
    pub body: Block,
}

impl ConstructorDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UnnamedFunctionDefinition = Rc<UnnamedFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct UnnamedFunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: UnnamedFunctionAttributes,
    pub body: FunctionBody,
}

impl UnnamedFunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FallbackFunctionDefinition = Rc<FallbackFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct FallbackFunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: FallbackFunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

impl FallbackFunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ReceiveFunctionDefinition = Rc<ReceiveFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct ReceiveFunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: ReceiveFunctionAttributes,
    pub body: FunctionBody,
}

impl ReceiveFunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ModifierDefinition = Rc<ModifierDefinitionStruct>;

#[derive(Debug)]
pub struct ModifierDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub parameters: Option<ParametersDeclaration>,
    pub attributes: ModifierAttributes,
    pub body: FunctionBody,
}

impl ModifierDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Debug)]
pub struct ModifierInvocationStruct {
    pub node: Rc<SyntaxNode>,
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

impl ModifierInvocationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Debug)]
pub struct EventDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub parameters: EventParametersDeclaration,
    pub anonymous_keyword: Option<Rc<TerminalNode>>,
}

impl EventDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EventParametersDeclaration = Rc<EventParametersDeclarationStruct>;

#[derive(Debug)]
pub struct EventParametersDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: EventParameters,
}

impl EventParametersDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Debug)]
pub struct EventParameterStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub indexed_keyword: Option<Rc<TerminalNode>>,
    pub name: Option<Rc<TerminalNode>>,
}

impl EventParameterStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub value_type: ElementaryType,
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Debug)]
pub struct ErrorDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub members: ErrorParametersDeclaration,
}

impl ErrorDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ErrorParametersDeclaration = Rc<ErrorParametersDeclarationStruct>;

#[derive(Debug)]
pub struct ErrorParametersDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: ErrorParameters,
}

impl ErrorParametersDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Debug)]
pub struct ErrorParameterStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
}

impl ErrorParameterStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Debug)]
pub struct ArrayTypeNameStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: TypeName,
    pub index: Option<Expression>,
}

impl ArrayTypeNameStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Debug)]
pub struct FunctionTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionTypeAttributes,
    pub returns: Option<ReturnsDeclaration>,
}

impl FunctionTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Debug)]
pub struct MappingTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub key_type: MappingKey,
    pub value_type: MappingValue,
}

impl MappingTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Debug)]
pub struct MappingKeyStruct {
    pub node: Rc<SyntaxNode>,
    pub key_type: MappingKeyType,
    pub name: Option<Rc<TerminalNode>>,
}

impl MappingKeyStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Debug)]
pub struct MappingValueStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub name: Option<Rc<TerminalNode>>,
}

impl MappingValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Debug)]
pub struct AddressTypeStruct {
    pub node: Rc<SyntaxNode>,
    pub payable_keyword: Option<Rc<TerminalNode>>,
}

impl AddressTypeStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type Block = Rc<BlockStruct>;

#[derive(Debug)]
pub struct BlockStruct {
    pub node: Rc<SyntaxNode>,
    pub statements: Statements,
}

impl BlockStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Debug)]
pub struct UncheckedBlockStruct {
    pub node: Rc<SyntaxNode>,
    pub block: Block,
}

impl UncheckedBlockStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Debug)]
pub struct ExpressionStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Expression,
}

impl ExpressionStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Debug)]
pub struct AssemblyStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub label: Option<StringLiteral>,
    pub flags: Option<AssemblyFlagsDeclaration>,
    pub body: YulBlock,
}

impl AssemblyStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AssemblyFlagsDeclaration = Rc<AssemblyFlagsDeclarationStruct>;

#[derive(Debug)]
pub struct AssemblyFlagsDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub flags: AssemblyFlags,
}

impl AssemblyFlagsDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub var_keyword: Option<Rc<TerminalNode>>,
    pub elements: TupleDeconstructionElements,
    pub expression: Expression,
}

impl TupleDeconstructionStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Debug)]
pub struct TupleDeconstructionElementStruct {
    pub node: Rc<SyntaxNode>,
    pub member: Option<TupleMember>,
}

impl TupleDeconstructionElementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Debug)]
pub struct TypedTupleMemberStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

impl TypedTupleMemberStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Debug)]
pub struct UntypedTupleMemberStruct {
    pub node: Rc<SyntaxNode>,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
}

impl UntypedTupleMemberStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Rc<TerminalNode>,
    pub value: Option<VariableDeclarationValue>,
}

impl VariableDeclarationStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type VariableDeclarationValue = Rc<VariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct VariableDeclarationValueStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Expression,
}

impl VariableDeclarationValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Debug)]
pub struct IfStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub condition: Expression,
    pub body: Statement,
    pub else_branch: Option<ElseBranch>,
}

impl IfStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ElseBranch = Rc<ElseBranchStruct>;

#[derive(Debug)]
pub struct ElseBranchStruct {
    pub node: Rc<SyntaxNode>,
    pub body: Statement,
}

impl ElseBranchStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Debug)]
pub struct ForStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub body: Statement,
}

impl ForStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Debug)]
pub struct WhileStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub condition: Expression,
    pub body: Statement,
}

impl WhileStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Debug)]
pub struct DoWhileStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub body: Statement,
    pub condition: Expression,
}

impl DoWhileStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Debug)]
pub struct ContinueStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl ContinueStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Debug)]
pub struct BreakStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl BreakStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Debug)]
pub struct ReturnStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Option<Expression>,
}

impl ReturnStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Debug)]
pub struct EmitStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
}

impl EmitStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Debug)]
pub struct TryStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Expression,
    pub returns: Option<ReturnsDeclaration>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

impl TryStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Debug)]
pub struct CatchClauseStruct {
    pub node: Rc<SyntaxNode>,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

impl CatchClauseStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Debug)]
pub struct CatchClauseErrorStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Option<Rc<TerminalNode>>,
    pub parameters: ParametersDeclaration,
}

impl CatchClauseErrorStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Debug)]
pub struct RevertStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub error: Option<IdentifierPath>,
    pub arguments: ArgumentsDeclaration,
}

impl RevertStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Debug)]
pub struct ThrowStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl ThrowStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl AssignmentExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Debug)]
pub struct ConditionalExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub true_expression: Expression,
    pub false_expression: Expression,
}

impl ConditionalExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Debug)]
pub struct OrExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl OrExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Debug)]
pub struct AndExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl AndExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Debug)]
pub struct EqualityExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl EqualityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Debug)]
pub struct InequalityExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl InequalityExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseOrExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseOrExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseXorExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseXorExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Debug)]
pub struct BitwiseAndExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub right_operand: Expression,
}

impl BitwiseAndExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Debug)]
pub struct ShiftExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl ShiftExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl AdditiveExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl MultiplicativeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Debug)]
pub struct ExponentiationExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

impl ExponentiationExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Debug)]
pub struct PostfixExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub operator: Rc<TerminalNode>,
}

impl PostfixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Debug)]
pub struct PrefixExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operator: Rc<TerminalNode>,
    pub operand: Expression,
}

impl PrefixExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct FunctionCallExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

impl FunctionCallExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Debug)]
pub struct CallOptionsExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub options: CallOptions,
}

impl CallOptionsExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub member: Rc<TerminalNode>,
}

impl MemberAccessExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Debug)]
pub struct IndexAccessExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: Expression,
    pub start: Option<Expression>,
    pub end: Option<IndexAccessEnd>,
}

impl IndexAccessExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type IndexAccessEnd = Rc<IndexAccessEndStruct>;

#[derive(Debug)]
pub struct IndexAccessEndStruct {
    pub node: Rc<SyntaxNode>,
    pub end: Option<Expression>,
}

impl IndexAccessEndStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type PositionalArgumentsDeclaration = Rc<PositionalArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct PositionalArgumentsDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub arguments: PositionalArguments,
}

impl PositionalArgumentsDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NamedArgumentsDeclaration = Rc<NamedArgumentsDeclarationStruct>;

#[derive(Debug)]
pub struct NamedArgumentsDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub arguments: Option<NamedArgumentGroup>,
}

impl NamedArgumentsDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NamedArgumentGroup = Rc<NamedArgumentGroupStruct>;

#[derive(Debug)]
pub struct NamedArgumentGroupStruct {
    pub node: Rc<SyntaxNode>,
    pub arguments: NamedArguments,
}

impl NamedArgumentGroupStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Debug)]
pub struct NamedArgumentStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub value: Expression,
}

impl NamedArgumentStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Debug)]
pub struct TypeExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
}

impl TypeExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Debug)]
pub struct NewExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub type_name: TypeName,
}

impl NewExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Debug)]
pub struct TupleExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub items: TupleValues,
}

impl TupleExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Debug)]
pub struct TupleValueStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: Option<Expression>,
}

impl TupleValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Debug)]
pub struct ArrayExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub items: ArrayValues,
}

impl ArrayExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Debug)]
pub struct HexNumberExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

impl HexNumberExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Debug)]
pub struct DecimalNumberExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub literal: Rc<TerminalNode>,
    pub unit: Option<NumberUnit>,
}

impl DecimalNumberExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Debug)]
pub struct YulBlockStruct {
    pub node: Rc<SyntaxNode>,
    pub statements: YulStatements,
}

impl YulBlockStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Debug)]
pub struct YulFunctionDefinitionStruct {
    pub node: Rc<SyntaxNode>,
    pub name: Rc<TerminalNode>,
    pub parameters: YulParametersDeclaration,
    pub returns: Option<YulReturnsDeclaration>,
    pub body: YulBlock,
}

impl YulFunctionDefinitionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulParametersDeclaration = Rc<YulParametersDeclarationStruct>;

#[derive(Debug)]
pub struct YulParametersDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub parameters: YulParameters,
}

impl YulParametersDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulReturnsDeclaration = Rc<YulReturnsDeclarationStruct>;

#[derive(Debug)]
pub struct YulReturnsDeclarationStruct {
    pub node: Rc<SyntaxNode>,
    pub variables: YulVariableNames,
}

impl YulReturnsDeclarationStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

impl YulVariableDeclarationStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Debug)]
pub struct YulVariableDeclarationValueStruct {
    pub node: Rc<SyntaxNode>,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

impl YulVariableDeclarationValueStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulVariableAssignmentStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

impl YulVariableAssignmentStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Debug)]
pub struct YulColonAndEqualStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulColonAndEqualStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Debug)]
pub struct YulStackAssignmentStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub assignment: YulStackAssignmentOperator,
    pub variable: Rc<TerminalNode>,
}

impl YulStackAssignmentStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Debug)]
pub struct YulEqualAndColonStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulEqualAndColonStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Debug)]
pub struct YulIfStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub condition: YulExpression,
    pub body: YulBlock,
}

impl YulIfStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Debug)]
pub struct YulForStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

impl YulForStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Debug)]
pub struct YulSwitchStatementStruct {
    pub node: Rc<SyntaxNode>,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

impl YulSwitchStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Debug)]
pub struct YulDefaultCaseStruct {
    pub node: Rc<SyntaxNode>,
    pub body: YulBlock,
}

impl YulDefaultCaseStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Debug)]
pub struct YulValueCaseStruct {
    pub node: Rc<SyntaxNode>,
    pub value: YulLiteral,
    pub body: YulBlock,
}

impl YulValueCaseStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Debug)]
pub struct YulLeaveStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulLeaveStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Debug)]
pub struct YulBreakStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulBreakStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Debug)]
pub struct YulContinueStatementStruct {
    pub node: Rc<SyntaxNode>,
}

impl YulContinueStatementStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Debug)]
pub struct YulLabelStruct {
    pub node: Rc<SyntaxNode>,
    pub label: Rc<TerminalNode>,
}

impl YulLabelStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Debug)]
pub struct YulFunctionCallExpressionStruct {
    pub node: Rc<SyntaxNode>,
    pub operand: YulExpression,
    pub arguments: YulArguments,
}

impl YulFunctionCallExpressionStruct {
    pub fn id(&self) -> NodeId {
        self.node.id()
    }
}

//
// Choices:
//

#[derive(Debug)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    InterfaceDefinition(InterfaceDefinition),
    LibraryDefinition(LibraryDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    FunctionDefinition(FunctionDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingDirective(UsingDirective),
    EventDefinition(EventDefinition),
    ConstantDefinition(ConstantDefinition),
}

#[derive(Debug)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

#[derive(Debug)]
pub enum AbicoderVersion {
    AbicoderV1Keyword,
    AbicoderV2Keyword,
}

#[derive(Debug)]
pub enum ExperimentalFeature {
    StringLiteral(StringLiteral),
    ABIEncoderV2Keyword,
    SMTCheckerKeyword,
}

#[derive(Debug)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

#[derive(Debug)]
pub enum VersionOperator {
    Caret,
    Tilde,
    Equal,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

#[derive(Debug)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    SingleQuotedVersionLiteral(Rc<TerminalNode>),
    DoubleQuotedVersionLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum ImportClause {
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
}

#[derive(Debug)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

#[derive(Debug)]
pub enum UsingOperator {
    Ampersand,
    Asterisk,
    BangEqual,
    Bar,
    Caret,
    EqualEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Minus,
    Percent,
    Plus,
    Slash,
    Tilde,
}

#[derive(Debug)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk,
}

#[derive(Debug)]
pub enum ContractSpecifier {
    InheritanceSpecifier(InheritanceSpecifier),
    StorageLayoutSpecifier(StorageLayoutSpecifier),
}

#[derive(Debug)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    ConstructorDefinition(ConstructorDefinition),
    ReceiveFunctionDefinition(ReceiveFunctionDefinition),
    FallbackFunctionDefinition(FallbackFunctionDefinition),
    UnnamedFunctionDefinition(UnnamedFunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
}

#[derive(Debug)]
pub enum StateVariableAttribute {
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword,
    InternalKeyword,
    PrivateKeyword,
    PublicKeyword,
    ImmutableKeyword,
    TransientKeyword,
}

#[derive(Debug)]
pub enum FunctionName {
    Identifier(Rc<TerminalNode>),
    FallbackKeyword,
    ReceiveKeyword,
}

#[derive(Debug)]
pub enum FunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword,
    ExternalKeyword,
    InternalKeyword,
    PayableKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    ViewKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum FunctionBody {
    Block(Block),
    Semicolon,
}

#[derive(Debug)]
pub enum ConstructorAttribute {
    ModifierInvocation(ModifierInvocation),
    InternalKeyword,
    OverrideKeyword,
    PayableKeyword,
    PublicKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum UnnamedFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    ConstantKeyword,
    ExternalKeyword,
    InternalKeyword,
    PayableKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    ViewKeyword,
}

#[derive(Debug)]
pub enum FallbackFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword,
    PayableKeyword,
    PureKeyword,
    ViewKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword,
    PayableKeyword,
    VirtualKeyword,
}

#[derive(Debug)]
pub enum ModifierAttribute {
    OverrideSpecifier(OverrideSpecifier),
    VirtualKeyword,
}

#[derive(Debug)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug)]
pub enum FunctionTypeAttribute {
    InternalKeyword,
    ExternalKeyword,
    PrivateKeyword,
    PublicKeyword,
    ConstantKeyword,
    PureKeyword,
    ViewKeyword,
    PayableKeyword,
}

#[derive(Debug)]
pub enum MappingKeyType {
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

#[derive(Debug)]
pub enum ElementaryType {
    AddressType(AddressType),
    BytesKeyword(Rc<TerminalNode>),
    IntKeyword(Rc<TerminalNode>),
    UintKeyword(Rc<TerminalNode>),
    FixedKeyword(Rc<TerminalNode>),
    UfixedKeyword(Rc<TerminalNode>),
    BoolKeyword,
    ByteKeyword,
    StringKeyword,
}

#[derive(Debug)]
pub enum Statement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
    AssemblyStatement(AssemblyStatement),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug)]
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

#[derive(Debug)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    VarKeyword,
}

#[derive(Debug)]
pub enum StorageLocation {
    MemoryKeyword,
    StorageKeyword,
    CallDataKeyword,
}

#[derive(Debug)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Debug)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon,
}

#[derive(Debug)]
pub enum Expression {
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    OrExpression(OrExpression),
    AndExpression(AndExpression),
    EqualityExpression(EqualityExpression),
    InequalityExpression(InequalityExpression),
    BitwiseOrExpression(BitwiseOrExpression),
    BitwiseXorExpression(BitwiseXorExpression),
    BitwiseAndExpression(BitwiseAndExpression),
    ShiftExpression(ShiftExpression),
    AdditiveExpression(AdditiveExpression),
    MultiplicativeExpression(MultiplicativeExpression),
    ExponentiationExpression(ExponentiationExpression),
    PostfixExpression(PostfixExpression),
    PrefixExpression(PrefixExpression),
    FunctionCallExpression(FunctionCallExpression),
    CallOptionsExpression(CallOptionsExpression),
    MemberAccessExpression(MemberAccessExpression),
    IndexAccessExpression(IndexAccessExpression),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    TypeExpression(TypeExpression),
    ArrayExpression(ArrayExpression),
    HexNumberExpression(HexNumberExpression),
    DecimalNumberExpression(DecimalNumberExpression),
    StringExpression(StringExpression),
    ElementaryType(ElementaryType),
    Identifier(Rc<TerminalNode>),
    PayableKeyword,
    ThisKeyword,
    SuperKeyword,
    TrueKeyword,
    FalseKeyword,
}

#[derive(Debug)]
pub enum ArgumentsDeclaration {
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration),
}

#[derive(Debug)]
pub enum NumberUnit {
    WeiKeyword,
    GweiKeyword,
    SzaboKeyword,
    FinneyKeyword,
    EtherKeyword,
    SecondsKeyword,
    MinutesKeyword,
    HoursKeyword,
    DaysKeyword,
    WeeksKeyword,
    YearsKeyword,
}

#[derive(Debug)]
pub enum StringExpression {
    StringLiteral(StringLiteral),
    StringLiterals(StringLiterals),
    HexStringLiteral(HexStringLiteral),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

#[derive(Debug)]
pub enum StringLiteral {
    SingleQuotedStringLiteral(Rc<TerminalNode>),
    DoubleQuotedStringLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum HexStringLiteral {
    SingleQuotedHexStringLiteral(Rc<TerminalNode>),
    DoubleQuotedHexStringLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum UnicodeStringLiteral {
    SingleQuotedUnicodeStringLiteral(Rc<TerminalNode>),
    DoubleQuotedUnicodeStringLiteral(Rc<TerminalNode>),
}

#[derive(Debug)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulStackAssignmentStatement(YulStackAssignmentStatement),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulLabel(YulLabel),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulExpression(YulExpression),
}

#[derive(Debug)]
pub enum YulAssignmentOperator {
    YulColonAndEqual(YulColonAndEqual),
    ColonEqual,
}

#[derive(Debug)]
pub enum YulStackAssignmentOperator {
    YulEqualAndColon(YulEqualAndColon),
    EqualColon,
}

#[derive(Debug)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

#[derive(Debug)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

#[derive(Debug)]
pub enum YulLiteral {
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
    YulDecimalLiteral(Rc<TerminalNode>),
    YulHexLiteral(Rc<TerminalNode>),
    YulTrueKeyword,
    YulFalseKeyword,
}

//
// Repeated & Separated
//

pub type SourceUnitMembers = Vec<SourceUnitMember>;

pub type VersionExpressionSets = Vec<VersionExpressionSet>;

pub type VersionExpressionSet = Vec<VersionExpression>;

pub type SimpleVersionLiteral = Vec<Rc<TerminalNode>>;

pub type ImportDeconstructionSymbols = Vec<ImportDeconstructionSymbol>;

pub type UsingDeconstructionSymbols = Vec<UsingDeconstructionSymbol>;

pub type ContractSpecifiers = Vec<ContractSpecifier>;

pub type InheritanceTypes = Vec<InheritanceType>;

pub type ContractMembers = Vec<ContractMember>;

pub type InterfaceMembers = Vec<ContractMember>;

pub type LibraryMembers = Vec<ContractMember>;

pub type StructMembers = Vec<StructMember>;

pub type EnumMembers = Vec<Rc<TerminalNode>>;

pub type StateVariableAttributes = Vec<StateVariableAttribute>;

pub type Parameters = Vec<Parameter>;

pub type FunctionAttributes = Vec<FunctionAttribute>;

pub type OverridePaths = Vec<IdentifierPath>;

pub type ConstructorAttributes = Vec<ConstructorAttribute>;

pub type UnnamedFunctionAttributes = Vec<UnnamedFunctionAttribute>;

pub type FallbackFunctionAttributes = Vec<FallbackFunctionAttribute>;

pub type ReceiveFunctionAttributes = Vec<ReceiveFunctionAttribute>;

pub type ModifierAttributes = Vec<ModifierAttribute>;

pub type EventParameters = Vec<EventParameter>;

pub type ErrorParameters = Vec<ErrorParameter>;

pub type FunctionTypeAttributes = Vec<FunctionTypeAttribute>;

pub type Statements = Vec<Statement>;

pub type AssemblyFlags = Vec<StringLiteral>;

pub type TupleDeconstructionElements = Vec<TupleDeconstructionElement>;

pub type CatchClauses = Vec<CatchClause>;

pub type PositionalArguments = Vec<Expression>;

pub type NamedArguments = Vec<NamedArgument>;

pub type CallOptions = Vec<NamedArgument>;

pub type TupleValues = Vec<TupleValue>;

pub type ArrayValues = Vec<Expression>;

pub type StringLiterals = Vec<StringLiteral>;

pub type HexStringLiterals = Vec<HexStringLiteral>;

pub type UnicodeStringLiterals = Vec<UnicodeStringLiteral>;

pub type IdentifierPath = Vec<Rc<TerminalNode>>;

pub type YulStatements = Vec<YulStatement>;

pub type YulParameters = Vec<Rc<TerminalNode>>;

pub type YulVariableNames = Vec<Rc<TerminalNode>>;

pub type YulSwitchCases = Vec<YulSwitchCase>;

pub type YulArguments = Vec<YulExpression>;

pub type YulPaths = Vec<YulPath>;

pub type YulPath = Vec<Rc<TerminalNode>>;
