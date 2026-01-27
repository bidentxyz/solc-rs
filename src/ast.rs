//! Solidity AST node types.
//!
//! This module provides strongly typed representations of Solidity's Abstract
//! Syntax Tree (AST) as output by the solc compiler. Each node type corresponds
//! to a Solidity language construct.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceUnit {
    pub id: i64,
    pub absolute_path: PathBuf,
    pub exported_symbols: HashMap<String, Vec<i64>>,
    pub src: SourceLocation,
    pub nodes: Vec<SourceUnitNode>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PragmaDirective {
    pub id: i64,
    pub literals: Vec<String>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub name_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolAlias {
    pub foreign: Identifier,
    pub local: Option<String>,
    pub name_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    pub id: i64,
    pub name: String,
    pub r#abstract: bool,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub canonical_name: String,
    pub contract_kind: ContractKind,
    pub fully_implemented: bool,
    pub linearized_base_contracts: Vec<i64>,
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: i64,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
    pub contract_dependencies: Vec<i64>,
    pub name_location: String,
    pub used_errors: Vec<i64>,
    pub used_events: Vec<i64>,
    #[serde(rename = "internalFunctionIDs")]
    pub internal_function_ids: Option<HashMap<String, i64>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InheritanceSpecifier {
    pub id: i64,
    pub base_name: IdentifierPath,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclaration {
    pub id: i64,
    pub name: String,
    pub type_name: TypeName,
    pub src: SourceLocation,
    pub name_location: String,
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub state_variable: bool,
    pub storage_location: StorageLocation,
    pub constant: bool,
    pub indexed: Option<bool>,
    pub value: Option<Box<Expression>>,
    pub documentation: Option<Documentation>,
    pub type_descriptions: TypeDescriptions,
    pub overrides: Option<OverrideSpecifier>,
    pub scope: i64,
    pub base_functions: Option<Vec<i64>>,
    pub function_selector: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverrideSpecifier {
    pub id: i64,
    pub overrides: Vec<IdentifierPath>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDefinition {
    pub id: i64,
    pub name: String,
    pub r#virtual: bool,
    pub kind: FunctionKind,
    pub visibility: Visibility,
    pub state_mutability: StateMutability,
    /// Present only when implemented=true
    pub body: Option<Block>,
    pub parameters: ParameterList,
    pub return_parameters: ParameterList,
    pub modifiers: Vec<ModifierInvocation>,
    pub src: SourceLocation,
    pub scope: i64,
    pub implemented: bool,
    pub documentation: Option<Documentation>,
    /// Present only when overrides base
    pub overrides: Option<OverrideSpecifier>,
    /// Present only when overrides/implements base
    pub base_functions: Option<Vec<i64>>,
    /// Present only on external/public functions
    pub function_selector: Option<String>,
    pub name_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionKind {
    Constructor,
    Function,
    Receive,
    Fallback,
    #[serde(rename = "freeFunction")]
    FreeFunction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    External,
    Public,
    Internal,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Pure,
    View,
    Nonpayable,
    Payable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifierInvocation {
    pub id: i64,
    pub kind: ModifierInvocationKind,
    pub modifier_name: IdentifierPath,
    pub arguments: Option<Vec<Box<Expression>>>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    Modifier,
    BaseConstructorSpecifier,
    ModifierInvocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterList {
    pub id: i64,
    pub parameters: Vec<VariableDeclaration>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifierDefinition {
    pub id: i64,
    pub name: String,
    pub r#virtual: bool,
    pub visibility: Visibility,
    pub parameters: ParameterList,
    pub body: Block,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
    pub name_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
    pub id: i64,
    pub name: String,
    pub anonymous: bool,
    pub event_selector: String,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    pub name_location: String,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDefinition {
    pub id: i64,
    pub name: String,
    pub error_selector: String,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    pub name_location: String,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructDefinition {
    pub id: i64,
    pub name: String,
    pub members: Vec<VariableDeclaration>,
    pub src: SourceLocation,
    pub scope: i64,
    pub documentation: Option<Documentation>,
    pub canonical_name: String,
    pub visibility: Visibility,
    pub name_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub id: i64,
    pub name: String,
    pub members: Vec<EnumValue>,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
    pub canonical_name: String,
    pub name_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub id: i64,
    pub name: String,
    pub name_location: String,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedValueTypeDefinition {
    pub id: i64,
    pub name: String,
    pub src: SourceLocation,
    pub canonical_name: String,
    pub name_location: String,
    pub underlying_type: TypeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsingForDirective {
    pub id: i64,
    pub library_name: IdentifierPath,
    pub type_name: Option<TypeName>,
    pub src: SourceLocation,
    pub global: bool,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: i64,
    pub statements: Vec<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UncheckedBlock {
    pub id: i64,
    pub statements: Vec<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IfStatement {
    pub id: i64,
    pub condition: Box<Expression>,
    pub true_body: Box<Statement>,
    pub false_body: Option<Box<Statement>>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForStatement {
    pub id: i64,
    pub initialization_expression: Box<Expression>,
    pub condition: Box<Expression>,
    pub loop_expression: Option<Box<Expression>>,
    pub body: Box<Statement>,
    pub src: SourceLocation,
    pub is_simple_counter_loop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhileStatement {
    pub id: i64,
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoWhileStatement {
    pub id: i64,
    pub condition: Box<Expression>,
    pub body: Box<Statement>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Continue {
    pub id: i64,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Break {
    pub id: i64,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    pub id: i64,
    pub function_return_parameters: i64,
    pub expression: Option<Box<Expression>>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmitStatement {
    pub id: i64,
    pub event_call: FunctionCall,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevertStatement {
    pub id: i64,
    pub error_call: FunctionCall,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TryStatement {
    pub id: i64,
    pub external_call: Box<Expression>,
    pub clauses: Vec<TryCatchClause>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TryCatchClause {
    pub id: i64,
    pub error_name: Option<String>,
    pub parameters: Option<ParameterList>,
    pub block: Block,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionStatement {
    pub id: i64,
    pub expression: Box<Expression>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableDeclarationStatement {
    pub id: i64,
    pub assignments: Vec<Option<i64>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    pub initial_value: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineAssembly {
    pub id: i64,
    #[serde(rename = "AST")]
    pub ast: YulBlock,
    pub external_references: Vec<ExternalReference>,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
    pub flags: Option<Vec<String>>,
    pub evm_version: String,
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
    YulVariableDeclaration(YulVariableDeclaration),
    YulFunctionDefinition(YulFunctionDefinition),
    YulExpressionStatement(YulExpressionStatement),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulBlock {
    pub src: String,
    pub native_src: String,
    pub statements: Vec<YulStatement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulAssignment {
    pub src: String,
    pub native_src: String,
    pub variable_names: Vec<YulIdentifier>,
    pub value: YulExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulVariableDeclaration {
    pub src: String,
    pub native_src: String,
    pub variables: Vec<YulTypedName>,
    pub value: YulExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulTypedName {
    pub name: String,
    pub src: String,
    pub native_src: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulExpressionStatement {
    pub src: String,
    pub native_src: String,
    pub expression: YulExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulIf {
    pub src: String,
    pub native_src: String,
    pub condition: YulExpression,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulForLoop {
    pub src: String,
    pub native_src: String,
    pub pre: YulBlock,
    pub condition: YulExpression,
    pub post: YulBlock,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulSwitch {
    pub src: String,
    pub native_src: String,
    pub expression: YulExpression,
    pub cases: Vec<YulCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulCase {
    pub src: String,
    pub native_src: String,
    pub value: YulCaseValue,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum YulCaseValue {
    String(String),
    Literal(YulLiteral),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulLiteral {
    pub src: String,
    pub native_src: String,
    pub kind: String,
    pub value: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionDefinition {
    pub src: String,
    pub native_src: String,
    pub name: String,
    pub parameters: Vec<YulTypedName>,
    pub body: YulBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulBreak {
    pub src: String,
    pub native_src: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum YulExpression {
    YulIdentifier(YulIdentifier),
    YulLiteral(YulLiteral),
    YulFunctionCall(YulFunctionCall),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulFunctionCall {
    pub src: String,
    pub native_src: String,
    pub function_name: Box<YulExpression>,
    pub arguments: Vec<YulExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulIdentifier {
    pub src: String,
    pub native_src: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalReference {
    pub declaration: i64,
    pub is_offset: bool,
    pub is_slot: bool,
    pub src: SourceLocation,
    pub value_size: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignmentOperator {
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
    #[serde(rename = "^=")]
    BitwiseXorAssign,
    #[serde(rename = "|=")]
    BitwiseOrAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryOperation {
    pub id: i64,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
    pub operator: BinaryOperator,
    pub common_type: Option<CommonType>,
    pub src: SourceLocation,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCall {
    pub id: i64,
    pub expression: Box<FunctionCallExpression>,
    pub arguments: Vec<Box<Expression>>,
    pub names: Vec<String>,
    pub kind: String,
    pub src: SourceLocation,
    pub try_call: bool,
    pub name_locations: Option<Vec<String>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
    pub type_descriptions: TypeDescriptions,
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallOptions {
    pub id: i64,
    pub expression: Box<Expression>,
    pub names: Vec<String>,
    pub options: Vec<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccess {
    pub id: i64,
    pub expression: Box<Expression>,
    pub member_name: String,
    pub member_location: Option<String>,
    pub src: SourceLocation,
    pub referenced_declaration: Option<i64>,
    pub type_descriptions: TypeDescriptions,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexAccess {
    pub id: i64,
    pub base_expression: Box<Expression>,
    pub index_expression: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexRangeAccess {
    pub id: i64,
    pub base_expression: Box<Expression>,
    pub start_expression: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub id: i64,
    pub name: String,
    pub overloaded_declarations: Vec<i64>,
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierPath {
    pub id: i64,
    pub name: String,
    pub name_locations: Option<Vec<String>>,
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Literal {
    pub id: i64,
    pub kind: LiteralKind,
    pub value: String,
    pub hex_value: Option<String>,
    pub subdenomination: Option<String>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiteralKind {
    Bool,
    Number,
    String,
    HexString,
    UnicodeString,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewExpression {
    pub id: i64,
    pub type_name: TypeName,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
    pub argument_types: Option<Vec<TypeDescriptions>>,
    pub is_constant: bool,
    pub is_l_value: bool,
    pub is_pure: bool,
    pub l_value_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeNameExpression {
    pub id: i64,
    pub type_name: ElementaryTypeName,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementaryTypeName {
    pub id: i64,
    pub name: ElementaryType,
    pub src: SourceLocation,
    pub state_mutability: Option<String>,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefinedTypeName {
    pub id: i64,
    pub path_node: Option<IdentifierPath>,
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTypeName {
    pub id: i64,
    pub base_type: Box<TypeName>,
    pub length: Option<Box<Expression>>,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub id: i64,
    pub key_type: Box<TypeName>,
    #[serde(default)]
    pub key_name: String,
    #[serde(default)]
    pub key_name_location: String,
    pub value_type: Box<TypeName>,
    #[serde(default)]
    pub value_name: String,
    #[serde(default)]
    pub value_name_location: String,
    pub src: SourceLocation,
    pub type_descriptions: TypeDescriptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonType {
    pub type_identifier: String,
    pub type_string: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementaryType {
    Uint(u16),
    Int(u16),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Default,
    Memory,
    Storage,
    Calldata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Mutable,
    Immutable,
    Constant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDocumentation {
    pub id: i64,
    pub text: String,
    pub src: SourceLocation,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use rayon::prelude::*;
    use serde::de::IntoDeserializer;
    use serde_json::Value;
    use serde_path_to_error::deserialize;
    use walkdir::WalkDir;

    fn find_deserialization_error(content: &str) -> String {
        let value: Value = serde_json::from_str(content).expect("Failed to parse JSON");
        find_error_in_value(&value, "root")
    }

    fn find_error_in_value(value: &Value, json_path: &str) -> String {
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let result = find_error_in_value(val, &format!("{}.{}", json_path, key));
                if !result.is_empty() {
                    return result;
                }
            }

            if let Some(node_type) = obj.get("nodeType") {
                if let Some(type_str) = node_type.as_str() {
                    return try_parse_node(value, json_path, type_str);
                }
            }
        }

        if let Some(arr) = value.as_array() {
            for (i, item) in arr.iter().enumerate() {
                let result = find_error_in_value(item, &format!("{}[{}]", json_path, i));
                if !result.is_empty() {
                    return result;
                }
            }
        }

        String::new()
    }

    fn try_parse_node(value: &Value, json_path: &str, node_type: &str) -> String {
        let json_str = serde_json::to_string_pretty(value)
            .unwrap_or_else(|_| String::from("Could not serialize value"));

        macro_rules! try_parse {
            ($type:ty) => {
                match deserialize::<_, $type>(value.clone().into_deserializer()) {
                    Ok(_) => String::new(),
                    Err(err) => {
                        let field_path = err.path().to_string();
                        format!(
                            "Failed to parse {} at path '{}':\nField: '{}'\nError: {}\nJSON:\n{}",
                            node_type, json_path, field_path, err, json_str
                        )
                    }
                }
            };
        }

        match node_type {
            "Literal" => try_parse!(Literal),
            "Identifier" => try_parse!(Identifier),
            "BinaryOperation" => try_parse!(BinaryOperation),
            "UnaryOperation" => try_parse!(UnaryOperation),
            "MemberAccess" => try_parse!(MemberAccess),
            "IndexAccess" => try_parse!(IndexAccess),
            "FunctionCall" => try_parse!(FunctionCall),
            "Assignment" => try_parse!(Assignment),
            "Conditional" => try_parse!(Conditional),
            "TupleExpression" => try_parse!(TupleExpression),
            "VariableDeclaration" => try_parse!(VariableDeclaration),
            "Block" => try_parse!(Block),
            "IfStatement" => try_parse!(IfStatement),
            "ForStatement" => try_parse!(ForStatement),
            "WhileStatement" => try_parse!(WhileStatement),
            "Return" => try_parse!(Return),
            "Break" => try_parse!(Break),
            "Continue" => try_parse!(Continue),
            "VariableDeclarationStatement" => try_parse!(VariableDeclarationStatement),
            "EmitStatement" => try_parse!(EmitStatement),
            "RevertStatement" => try_parse!(RevertStatement),
            "TryStatement" => try_parse!(TryStatement),
            "UncheckedBlock" => try_parse!(UncheckedBlock),
            "InlineAssembly" => try_parse!(InlineAssembly),
            "PlaceholderStatement" => try_parse!(PlaceholderStatement),
            "NewExpression" => try_parse!(NewExpression),
            "ElementaryTypeNameExpression" => try_parse!(ElementaryTypeNameExpression),
            "ExpressionStatement" => try_parse!(ExpressionStatement),
            "ContractDefinition" => try_parse!(ContractDefinition),
            "StructDefinition" => try_parse!(StructDefinition),
            "EnumDefinition" => try_parse!(EnumDefinition),
            "ErrorDefinition" => try_parse!(ErrorDefinition),
            "EventDefinition" => try_parse!(EventDefinition),
            "FunctionDefinition" => try_parse!(FunctionDefinition),
            "ModifierDefinition" => try_parse!(ModifierDefinition),
            "UserDefinedValueTypeDefinition" => try_parse!(UserDefinedValueTypeDefinition),
            "ImportDirective" => try_parse!(ImportDirective),
            "PragmaDirective" => try_parse!(PragmaDirective),
            "UsingForDirective" => try_parse!(UsingForDirective),
            "DoWhileStatement" => try_parse!(DoWhileStatement),
            "SourceUnit" => try_parse!(SourceUnit),
            _ => String::new(),
        }
    }

    #[test]
    fn fixtures() {
        let entries: Vec<walkdir::DirEntry> = WalkDir::new("fixtures/ast")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |e| e == "json"))
            .collect();

        entries.par_iter().for_each(|entry| {
            let content = fs::read_to_string(entry.path()).expect("Failed to read fixture file");
            let result: Result<SourceUnit, serde_json::Error> = serde_json::from_str(&content);
            if let Err(_) = result {
                let error_msg = find_deserialization_error(&content);
                panic!("Failed to parse {:?}: {}", entry.path(), error_msg);
            }
        });
    }
}
