//! Solidity AST node types.
//!
//! This module provides strongly typed representations of Solidity's Abstract
//! Syntax Tree (AST) as output by the solc compiler. Each node type corresponds
//! to a Solidity language construct.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub id: i64,
    pub absolute_path: PathBuf,
    pub exported_symbols: HashMap<String, Vec<i64>>,
    pub src: SourceLocation,
    pub nodes: Vec<SourceUnitNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum SourceUnitNode {
    ContractDefinition(ContractDefinition),
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ImportDirective(ImportDirective),
    PragmaDirective(PragmaDirective),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}

impl Default for SourceUnitNode {
    fn default() -> Self {
        SourceUnitNode::PragmaDirective(PragmaDirective::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PragmaDirective {
    pub id: i64,
    pub literals: Vec<String>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImportDirective {
    pub id: i64,
    pub absolute_path: PathBuf,
    pub file: PathBuf,
    pub unit_alias: String,
    pub symbol_aliases: Vec<SymbolAlias>,
    pub scope: i64,
    pub source_unit: i64,
    pub src: SourceLocation,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Identifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: Option<String>,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub id: i64,
    pub name: String,
    /// Present in solc >= 0.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#abstract: Option<bool>,
    pub base_contracts: Vec<InheritanceSpecifier>,
    /// Present in solc >= 0.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_name: Option<String>,
    pub contract_kind: ContractKind,
    pub fully_implemented: bool,
    pub linearized_base_contracts: Vec<i64>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: i64,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    pub contract_dependencies: Vec<i64>,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
    /// Present in solc >= 0.8.4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_errors: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_events: Option<Vec<i64>>,
    #[serde(
        rename = "internalFunctionIDs",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_function_ids: Option<HashMap<String, i64>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    #[default]
    Contract,
    Interface,
    Library,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ContractDefinitionNode {
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}

impl Default for ContractDefinitionNode {
    fn default() -> Self {
        ContractDefinitionNode::VariableDeclaration(VariableDeclaration::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub id: i64,
    pub base_name: IdentifierPath,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub id: i64,
    pub name: String,
    pub type_name: TypeName,
    pub src: SourceLocation,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
    pub visibility: Visibility,
    /// Present in solc >= 0.6.5; older versions use `constant` instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<Mutability>,
    pub state_variable: bool,
    pub storage_location: StorageLocation,
    pub constant: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<Expression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    pub scope: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_functions: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_selector: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub id: i64,
    pub overrides: Vec<IdentifierPath>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    pub id: i64,
    pub name: String,
    /// Present in solc >= 0.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<bool>,
    /// Present in solc >= 0.5.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<FunctionKind>,
    pub visibility: Visibility,
    pub state_mutability: StateMutability,
    /// Present only when implemented=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Block>,
    pub parameters: ParameterList,
    pub return_parameters: ParameterList,
    pub modifiers: Vec<ModifierInvocation>,
    pub src: SourceLocation,
    pub scope: i64,
    pub implemented: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    /// Present only when overrides base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<OverrideSpecifier>,
    /// Present only when overrides/implements base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_functions: Option<Vec<i64>>,
    /// Present only on external/public functions; absent on constructors,
    /// receive, and fallback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_selector: Option<String>,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FunctionKind {
    Constructor,
    #[default]
    Function,
    Receive,
    Fallback,
    #[serde(rename = "freeFunction")]
    FreeFunction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    #[default]
    External,
    Public,
    Internal,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    #[default]
    Pure,
    View,
    Nonpayable,
    Payable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub id: i64,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ModifierInvocationKind>,
    pub modifier_name: IdentifierPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<Box<Expression>>>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    #[default]
    Modifier,
    BaseConstructorSpecifier,
    ModifierInvocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub id: i64,
    pub parameters: Vec<VariableDeclaration>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub id: i64,
    pub name: String,
    /// Present in solc >= 0.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#virtual: Option<bool>,
    pub visibility: Visibility,
    pub parameters: ParameterList,
    pub body: Block,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub id: i64,
    pub name: String,
    pub anonymous: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selector: Option<String>,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_selector: Option<String>,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub id: i64,
    pub name: String,
    pub members: Vec<VariableDeclaration>,
    pub src: SourceLocation,
    pub scope: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    pub canonical_name: String,
    pub visibility: Visibility,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub id: i64,
    pub name: String,
    pub members: Vec<EnumValue>,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    pub canonical_name: String,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub id: i64,
    pub name: String,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub id: i64,
    pub name: String,
    pub src: SourceLocation,
    pub canonical_name: String,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_location: Option<String>,
    pub underlying_type: TypeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub id: i64,
    pub library_name: IdentifierPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<TypeName>,
    pub src: SourceLocation,
    /// Present in solc >= 0.8.13
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Block),
    Break(Break),
    Continue(Continue),
    DoWhileStatement(DoWhileStatement),
    EmitStatement(EmitStatement),
    ExpressionStatement(ExpressionStatement),
    ForStatement(ForStatement),
    IfStatement(IfStatement),
    InlineAssembly(InlineAssembly),
    PlaceholderStatement(PlaceholderStatement),
    Return(Return),
    RevertStatement(RevertStatement),
    TryStatement(TryStatement),
    UncheckedBlock(UncheckedBlock),
    VariableDeclarationStatement(VariableDeclarationStatement),
    WhileStatement(WhileStatement),
}

impl Default for Statement {
    fn default() -> Self {
        Statement::Block(Block::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: i64,
    pub statements: Vec<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UncheckedBlock {
    pub id: i64,
    pub statements: Vec<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub id: i64,
    pub condition: Box<Expression>,
    pub true_body: Box<Statement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub false_body: Option<Box<Statement>>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_expression: Option<Box<Expression>>,
    pub condition: Box<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_expression: Option<Box<Expression>>,
    pub body: Box<Statement>,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_simple_counter_loop: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub id: i64,
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: i64,
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Continue {
    pub id: i64,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Break {
    pub id: i64,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: i64,
    pub function_return_parameters: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<Expression>>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub id: i64,
    pub event_call: FunctionCall,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub id: i64,
    pub error_call: FunctionCall,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub id: i64,
    pub external_call: Box<Expression>,
    pub clauses: Vec<TryCatchClause>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterList>,
    pub block: Block,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: i64,
    pub expression: Box<Expression>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub id: i64,
    pub assignments: Vec<Option<i64>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<Box<Expression>>,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    pub id: i64,
    /// Present in solc >= 0.6.0 as structured Yul AST
    #[serde(rename = "AST", skip_serializing_if = "Option::is_none")]
    pub ast: Option<YulBlock>,
    /// Present in solc < 0.6.0 as raw assembly string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<String>,
    pub external_references: Vec<ExternalReference>,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    /// Present in solc >= 0.8.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
#[allow(clippy::large_enum_variant)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulAssignment(YulAssignment),
    YulFunctionCall(YulFunctionCall),
    YulIf(YulIf),
    YulForLoop(YulForLoop),
    YulSwitch(YulSwitch),
    YulBreak(YulBreak),
    YulContinue(YulContinue),
    YulLeave(YulLeave),
    YulVariableDeclaration(YulVariableDeclaration),
    YulFunctionDefinition(YulFunctionDefinition),
    YulExpressionStatement(YulExpressionStatement),
}

impl Default for YulStatement {
    fn default() -> Self {
        YulStatement::YulBlock(YulBlock::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub statements: Vec<YulStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub variable_names: Vec<YulIdentifier>,
    pub value: YulExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub variables: Vec<YulTypedName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<YulExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub name: String,
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub expression: YulExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulIf {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub condition: YulExpression,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulForLoop {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub pre: YulBlock,
    pub condition: YulExpression,
    pub post: YulBlock,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub expression: YulExpression,
    pub cases: Vec<YulCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub value: YulCaseValue,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum YulCaseValue {
    String(String),
    Literal(YulLiteral),
}

impl Default for YulCaseValue {
    fn default() -> Self {
        YulCaseValue::String(String::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteral {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_value: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionDefinition {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<YulTypedName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub return_variables: Vec<YulTypedName>,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulBreak {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulContinue {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulLeave {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum YulExpression {
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
    YulFunctionCall(YulFunctionCall),
}

impl Default for YulExpression {
    fn default() -> Self {
        YulExpression::YulIdentifier(YulIdentifier::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub function_name: Box<YulExpression>,
    pub arguments: Vec<YulExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_src: Option<String>,
    pub name: String,
}

/// Yul object produced as `irAst` / `irOptimizedAst`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulObject {
    pub name: String,
    pub code: YulCode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_objects: Vec<YulSubObject>,
}

/// Code section of a [`YulObject`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulCode {
    pub block: YulBlock,
}

/// Nested object or data section inside a [`YulObject`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum YulSubObject {
    YulObject(YulObject),
    YulData(YulData),
}

impl Default for YulSubObject {
    fn default() -> Self {
        YulSubObject::YulData(YulData::default())
    }
}

/// Named data section inside a [`YulObject`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YulData {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExternalReference {
    /// Present in solc < 0.6.0 when references are keyed by variable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub declaration: i64,
    pub is_offset: bool,
    pub is_slot: bool,
    pub src: SourceLocation,
    pub value_size: i64,
}

impl<'de> Deserialize<'de> for ExternalReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Details {
            declaration: i64,
            is_offset: bool,
            is_slot: bool,
            src: SourceLocation,
            value_size: i64,
        }

        let value = serde_json::Value::deserialize(deserializer)?;

        // Modern format: flat object with a declaration field
        if value.get("declaration").is_some() {
            let details: Details =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            return Ok(ExternalReference {
                name: None,
                declaration: details.declaration,
                is_offset: details.is_offset,
                is_slot: details.is_slot,
                src: details.src,
                value_size: details.value_size,
            });
        }

        // Legacy format (solc < 0.6.0): { "varName": { declaration, ... } }
        let serde_json::Value::Object(obj) = value else {
            return Err(serde::de::Error::custom(
                "invalid external reference: expected flat object or single-key map",
            ));
        };

        if obj.len() != 1 {
            return Err(serde::de::Error::custom(
                "invalid external reference: expected flat object or single-key map",
            ));
        }

        let (name, details_value) = obj.into_iter().next().ok_or_else(|| {
            serde::de::Error::custom(
                "invalid external reference: expected flat object or single-key map",
            )
        })?;
        let details: Details =
            serde_json::from_value(details_value).map_err(serde::de::Error::custom)?;
        Ok(ExternalReference {
            name: Some(name),
            declaration: details.declaration,
            is_offset: details.is_offset,
            is_slot: details.is_slot,
            src: details.src,
            value_size: details.value_size,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceholderStatement {
    pub id: i64,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
#[allow(clippy::large_enum_variant)]
pub enum Expression {
    Assignment(Assignment),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(FunctionCall),
    Identifier(Identifier),
    IndexAccess(IndexAccess),
    IndexRangeAccess(IndexRangeAccess),
    Literal(Literal),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    UnaryOperation(UnaryOperation),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Literal(Literal::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AssignmentOperator {
    #[default]
    #[serde(rename = "*=")]
    MulAssign,
    #[serde(rename = "+=")]
    AddAssign,
    #[serde(rename = "-=")]
    SubAssign,
    #[serde(rename = "/=")]
    DivAssign,
    #[serde(rename = "<<=")]
    LeftShiftAssign,
    #[serde(rename = "=")]
    Assign,
    #[serde(rename = ">>=")]
    RightShiftAssign,
    #[serde(rename = "&=")]
    BitwiseAndAssign,
    #[serde(rename = "^=")]
    BitwiseXorAssign,
    #[serde(rename = "|=")]
    BitwiseOrAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub id: i64,
    pub left_hand_side: Box<Expression>,
    pub right_hand_side: Box<Expression>,
    pub operator: AssignmentOperator,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BinaryOperator {
    #[default]
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "%")]
    Modulo,
    #[serde(rename = "&")]
    BitwiseAnd,
    #[serde(rename = "&&")]
    LogicalAnd,
    #[serde(rename = "*")]
    Mul,
    #[serde(rename = "**")]
    Exp,
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "/")]
    Div,
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "<<")]
    LeftShift,
    #[serde(rename = "<=")]
    LessEqual,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = ">=")]
    GreaterEqual,
    #[serde(rename = ">>")]
    RightShift,
    #[serde(rename = "^")]
    BitwiseXor,
    #[serde(rename = "|")]
    BitwiseOr,
    #[serde(rename = "||")]
    LogicalOr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub id: i64,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operator: BinaryOperator,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_type: Option<CommonType>,
    pub src: SourceLocation,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Conditional {
    pub id: i64,
    pub condition: Box<Expression>,
    pub true_expression: Box<Expression>,
    pub false_expression: Box<Expression>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UnaryOperator {
    #[default]
    #[serde(rename = "!")]
    Not,
    #[serde(rename = "++")]
    Increment,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "--")]
    Decrement,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "~")]
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnaryOperation {
    pub id: i64,
    pub sub_expression: Box<Expression>,
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: i64,
    pub expression: Box<FunctionCallExpression>,
    pub arguments: Vec<Box<Expression>>,
    pub names: Vec<String>,
    pub kind: String,
    pub src: SourceLocation,
    /// Present in solc >= 0.6.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_call: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_locations: Option<Vec<String>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub id: i64,
    pub expression: Box<Expression>,
    pub names: Vec<String>,
    pub options: Vec<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum FunctionCallExpression {
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(FunctionCall),
    FunctionCallOptions(FunctionCallOptions),
    Identifier(Identifier),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
}

impl Default for FunctionCallExpression {
    fn default() -> Self {
        FunctionCallExpression::Identifier(Identifier::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub id: i64,
    pub expression: Box<Expression>,
    pub member_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_location: Option<String>,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_declaration: Option<i64>,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub id: i64,
    pub base_expression: Box<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_expression: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub id: i64,
    pub base_expression: Box<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_expression: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TupleExpression {
    pub id: i64,
    pub components: Vec<Option<Box<Expression>>>,
    pub src: SourceLocation,
    pub is_inline_array: bool,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: i64,
    pub name: String,
    pub overloaded_declarations: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub id: i64,
    pub kind: LiteralKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdenomination: Option<String>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum LiteralKind {
    #[default]
    Bool,
    Number,
    String,
    HexString,
    UnicodeString,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub id: i64,
    pub type_name: TypeName,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub id: i64,
    pub type_name: ElementaryTypeName,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    UserDefinedTypeName(UserDefinedTypeName),
}

impl Default for TypeName {
    fn default() -> Self {
        TypeName::ElementaryTypeName(ElementaryTypeName::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub id: i64,
    pub name: ElementaryType,
    pub src: SourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,
    pub type_descriptions: TypeDescriptions,
}

impl<'de> Deserialize<'de> for ElementaryTypeName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Full {
            id: i64,
            name: ElementaryType,
            src: SourceLocation,
            state_mutability: Option<String>,
            type_descriptions: TypeDescriptions,
        }

        let value = serde_json::Value::deserialize(deserializer)?;

        // Legacy format (solc < 0.6.0): bare type string, e.g. "bytes"
        if value.is_string() {
            let name: ElementaryType =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            return Ok(ElementaryTypeName {
                id: 0,
                name,
                src: SourceLocation::default(),
                state_mutability: None,
                type_descriptions: TypeDescriptions::default(),
            });
        }

        let full: Full = serde_json::from_value(value).map_err(serde::de::Error::custom)?;
        Ok(ElementaryTypeName {
            id: full.id,
            name: full.name,
            src: full.src,
            state_mutability: full.state_mutability,
            type_descriptions: full.type_descriptions,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub id: i64,
    /// Present in solc < 0.8.0; replaced by `path_node` in later versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Present in solc >= 0.8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_node: Option<IdentifierPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub id: i64,
    pub base_type: Box<TypeName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub id: i64,
    pub key_type: Box<TypeName>,
    /// Present in solc >= 0.8.18
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// Present in solc >= 0.8.18
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name_location: Option<String>,
    pub value_type: Box<TypeName>,
    /// Present in solc >= 0.8.18
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
    /// Present in solc >= 0.8.18
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_name_location: Option<String>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTypeName {
    pub id: i64,
    pub parameter_types: ParameterList,
    pub return_parameter_types: ParameterList,
    pub visibility: String,
    pub state_mutability: String,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceLocation {
    pub offset: usize,
    pub length: usize,
    pub source_index: usize,
}

impl Serialize for SourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{}:{}:{}",
            self.offset, self.length, self.source_index
        ))
    }
}

impl<'de> Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom(format!(
                "invalid source location: expected 'offset:length:sourceIndex', got '{}'",
                s
            )));
        }
        Ok(SourceLocation {
            offset: parts[0]
                .parse()
                .map_err(|e| serde::de::Error::custom(format!("invalid offset: {}", e)))?,
            length: parts[1]
                .parse()
                .map_err(|e| serde::de::Error::custom(format!("invalid length: {}", e)))?,
            source_index: parts[2]
                .parse()
                .map_err(|e| serde::de::Error::custom(format!("invalid source_index: {}", e)))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_string: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommonType {
    pub type_identifier: String,
    pub type_string: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ElementaryType {
    Uint(u16),
    Int(u16),
    #[default]
    Address,
    Payable,
    Bool,
    String,
    Bytes,
    FixedBytes(u16),
    Ufixed(u8, u8),
    Fixed(u8, u8),
}

impl<'de> Deserialize<'de> for ElementaryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "address" => Ok(Self::Address),
            "payable" => Ok(Self::Payable),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "bytes" => Ok(Self::Bytes),
            // Alias for bytes1, removed in Solidity 0.8.0.
            "byte" => Ok(Self::FixedBytes(1)),
            s if s.starts_with("uint") => {
                let bits = if s.len() == 4 {
                    256
                } else {
                    s[4..].parse::<u16>().map_err(serde::de::Error::custom)?
                };
                Ok(Self::Uint(bits))
            }
            s if s.starts_with("int") => {
                let bits = if s.len() == 3 {
                    256
                } else {
                    s[3..].parse::<u16>().map_err(serde::de::Error::custom)?
                };
                Ok(Self::Int(bits))
            }
            s if s.starts_with("bytes") => {
                let size = if s.len() == 5 {
                    0
                } else {
                    s[5..].parse::<u16>().map_err(serde::de::Error::custom)?
                };
                Ok(if size == 0 {
                    Self::Bytes
                } else {
                    Self::FixedBytes(size)
                })
            }
            _ => Err(serde::de::Error::custom(format!(
                "unknown elementary type: {}",
                s
            ))),
        }
    }
}

impl Serialize for ElementaryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Self::Uint(b) => format!("uint{}", b),
            Self::Int(b) => format!("int{}", b),
            Self::Address => "address".into(),
            Self::Payable => "payable".into(),
            Self::Bool => "bool".into(),
            Self::String => "string".into(),
            Self::Bytes => "bytes".into(),
            Self::FixedBytes(b) => format!("bytes{}", b),
            Self::Ufixed(t, f) => format!("ufixed{}x{}", t, f),
            Self::Fixed(t, f) => format!("fixed{}x{}", t, f),
        };
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Documentation {
    String(String),
    Structured(StructuredDocumentation),
}

impl Default for Documentation {
    fn default() -> Self {
        Documentation::String(String::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    #[default]
    Default,
    Memory,
    Storage,
    Calldata,
    Transient,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    #[default]
    Mutable,
    Immutable,
    Constant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub id: i64,
    pub text: String,
    pub src: SourceLocation,
}
