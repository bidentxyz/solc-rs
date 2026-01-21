//! Solidity AST node definitions.
//!
//! This module provides strongly typed representations of Solidity's Abstract
//! Syntax Tree (AST) as output by the solc compiler. Each node type corresponds
//! to a Solidity language construct.
//!
//! The Solidity compiler emits a detailed AST that represents all components of
//! Solidity source code, from type definitions to complex control structures.
//! This module models these nodes as Rust structs and enums with full serde
//! serialization support, enabling parsing, analysis, and transformation of
//! Solidity contracts.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceUnit {
    pub id: i64,
    #[serde(rename = "absolutePath")]
    pub absolute_path: String,
    #[serde(rename = "exportedSymbols", default)]
    pub exported_symbols: std::collections::HashMap<String, Vec<i64>>,
    pub src: SourceLocation,
    pub nodes: Vec<SourceUnitNode>,
    pub license: Option<String>,
}

/// Documentation can be either a plain string or a structured documentation object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Documentation {
    String(String),
    Structured(StructuredDocumentation),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
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
pub enum LiteralKind {
    Bool,
    Number,
    String,
    HexString,
    UnicodeString,
}

/// Source location information.
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

/// Source unit nodes.
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
#[serde(tag = "nodeType")]
pub enum ParameterListNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum BlockNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum UncheckedBlockNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum UsingForDirectiveNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ImportDirectiveNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum PragmaDirectiveNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum UserDefinedValueTypeDefinitionNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ModifierDefinitionNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum EnumDefinitionNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ErrorDefinitionNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum EventDefinitionNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum StructDefinitionNode {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum VariableDeclarationNode {}

/// Type descriptions provided by the compiler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDescriptions {
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,
    #[serde(rename = "typeString", skip_serializing_if = "Option::is_none")]
    pub type_string: Option<String>,
}

/// Common type for binary operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonType {
    #[serde(rename = "typeIdentifier")]
    pub type_identifier: String,
    #[serde(rename = "typeString")]
    pub type_string: String,
}

/// Elementary type names in Solidity.
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

/// Contract definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractDefinition {
    pub id: i64,
    pub name: String,
    #[serde(rename = "contractKind")]
    pub contract_kind: ContractKind,
    #[serde(deserialize_with = "deserialize_bool_or_int")]
    pub r#abstract: bool,
    #[serde(
        rename = "fullyImplemented",
        deserialize_with = "deserialize_bool_or_int"
    )]
    pub fully_implemented: bool,
    #[serde(rename = "linearizedBaseContracts")]
    pub linearized_base_contracts: Vec<i64>,
    #[serde(default)]
    pub nodes: Vec<ContractDefinitionNode>,
    pub scope: Option<i64>,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
    #[serde(rename = "baseContracts")]
    pub base_contracts: Option<Vec<InheritanceSpecifier>>,
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<String>,
    #[serde(rename = "contractDependencies")]
    pub contract_dependencies: Option<Vec<i64>>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    #[serde(rename = "usedErrors")]
    pub used_errors: Option<Vec<i64>>,
    #[serde(rename = "usedEvents")]
    pub used_events: Option<Vec<i64>>,
}

/// Contract nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum ContractDefinitionNode {
    EnumDefinition(EnumDefinition),
    ErrorDefinition(ErrorDefinition),
    EventDefinition(EventDefinition),
    FunctionDefinition(FunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    UsingForDirective(UsingForDirective),
    VariableDeclaration(VariableDeclaration),
}

/// Deserialize a boolean value that may be stored as an int (0 or 1) in JSON.
fn deserialize_bool_or_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    struct BoolOrIntVisitor;

    impl<'de> serde::de::Visitor<'de> for BoolOrIntVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or an integer (0 or 1)")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Error::custom(format!(
                    "invalid integer value for boolean: {}, expected 0 or 1",
                    value
                ))),
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Error::custom(format!(
                    "invalid integer value for boolean: {}, expected 0 or 1",
                    value
                ))),
            }
        }
    }

    deserializer.deserialize_any(BoolOrIntVisitor)
}

/// Variable declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub id: i64,
    pub name: String,
    #[serde(rename = "typeName")]
    pub type_name: Option<TypeName>,
    pub src: SourceLocation,
    // pub nodes: Vec<VariableDeclarationNode>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    pub visibility: Visibility,
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<StateMutability>,
    pub mutability: Option<Mutability>,
    #[serde(rename = "stateVariable")]
    pub state_variable: Option<bool>,
    #[serde(rename = "storageLocation")]
    pub storage_location: Option<StorageLocation>,
    pub constant: Option<bool>,
    pub immutable: Option<bool>,
    pub indexed: Option<bool>,
    pub value: Option<Expression>,
    pub documentation: Option<Documentation>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    pub overrides: Option<OverrideSpecifier>,
    pub scope: Option<i64>,
}

/// Binary operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryOperation {
    pub id: i64,
    #[serde(rename = "leftExpression")]
    pub left_expression: Expression,
    #[serde(rename = "rightExpression")]
    pub right_expression: Expression,
    pub operator: String,
    #[serde(rename = "commonType")]
    pub common_type: CommonType,
    pub src: SourceLocation,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Function call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub id: i64,
    pub expression: FunctionCallExpression,
    pub arguments: Vec<Expression>,
    pub names: Vec<String>,
    pub kind: String,
    pub src: SourceLocation,
    #[serde(rename = "tryCall")]
    pub try_call: bool,
    #[serde(rename = "nameLocations")]
    #[serde(default)]
    pub name_locations: Option<Vec<String>>,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "argumentTypes")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

/// If statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IfStatement {
    pub id: i64,
    pub condition: Expression,
    #[serde(rename = "trueBody")]
    pub true_body: Statement,
    #[serde(rename = "falseBody")]
    pub false_body: Option<Statement>,
    pub src: SourceLocation,
}

/// Block statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub id: i64,
    pub statements: Vec<Statement>,
    pub src: SourceLocation,
    #[serde(default)]
    pub nodes: Vec<BlockNode>,
}

/// Conditional expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conditional {
    pub id: i64,
    pub condition: Expression,
    #[serde(rename = "trueExpression")]
    pub true_expression: Expression,
    #[serde(rename = "falseExpression")]
    pub false_expression: Expression,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Variable declaration statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableDeclarationStatement {
    pub id: i64,
    pub assignments: Vec<Option<i64>>,
    pub declarations: Vec<Option<VariableDeclaration>>,
    #[serde(rename = "initialValue")]
    pub initial_value: Option<Expression>,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
}

/// Function definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub id: i64,
    pub name: String,
    pub r#virtual: bool,
    pub kind: FunctionKind,
    pub visibility: Visibility,
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
    pub body: Option<Block>,
    pub parameters: ParameterList,
    #[serde(rename = "returnParameters")]
    pub return_parameters: ParameterList,
    pub modifiers: Vec<ModifierInvocation>,
    pub src: SourceLocation,
    pub scope: i64,
    pub implemented: bool,
    pub documentation: Option<Documentation>,
    pub overrides: Option<OverrideSpecifier>,
    #[serde(rename = "baseFunctions")]
    pub base_functions: Option<Vec<i64>>,
    #[serde(rename = "functionSelector")]
    pub function_selector: Option<String>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    #[serde(default)]
    pub nodes: Vec<VariableDeclarationNode>,
}

/// Member access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberAccess {
    pub id: i64,
    pub expression: Expression,
    #[serde(rename = "memberName")]
    pub member_name: String,
    #[serde(rename = "memberLocation")]
    pub member_location: Option<String>,
    pub src: SourceLocation,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: Option<i64>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "argumentTypes")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
}

/// Unary operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnaryOperation {
    pub id: i64,
    #[serde(rename = "subExpression")]
    pub sub_expression: Expression,
    pub operator: String,
    #[serde(rename = "isPrefix", alias = "prefix")]
    pub is_prefix: bool,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
}

/// Assignment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    pub id: i64,
    #[serde(rename = "leftHandSide")]
    pub left_hand_side: Expression,
    #[serde(rename = "rightHandSide")]
    pub right_hand_side: Expression,
    pub operator: String,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Index access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexAccess {
    pub id: i64,
    #[serde(rename = "baseExpression")]
    pub base_expression: Expression,
    #[serde(rename = "indexExpression")]
    pub index_expression: Option<Expression>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Tuple expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TupleExpression {
    pub id: i64,
    pub components: Vec<Option<Expression>>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Return statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Return {
    pub id: i64,
    #[serde(rename = "functionReturnParameters")]
    pub function_return_parameters: i64,
    pub expression: Option<Expression>,
    pub src: SourceLocation,
}

/// Unchecked block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UncheckedBlock {
    pub id: i64,
    pub statements: Vec<Statement>,
    pub src: SourceLocation,
    #[serde(default)]
    pub nodes: Vec<UncheckedBlockNode>,
}

// Medium complexity

/// Expression statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub id: i64,
    pub expression: Expression,
    pub src: SourceLocation,
}

/// For statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForStatement {
    pub id: i64,
    #[serde(rename = "initializationExpression")]
    pub initialization_expression: Option<Expression>,
    pub condition: Option<Expression>,
    #[serde(rename = "loopExpression")]
    pub loop_expression: Option<Expression>,
    pub body: Statement,
    pub src: SourceLocation,
}

/// Mapping type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mapping {
    pub id: i64,
    #[serde(rename = "keyType")]
    pub key_type: TypeName,
    #[serde(rename = "valueType")]
    pub value_type: TypeName,
    pub src: SourceLocation,
}

/// Array type name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayTypeName {
    pub id: i64,
    #[serde(rename = "baseType")]
    pub base_type: TypeName,
    pub length: Option<Expression>,
    pub src: SourceLocation,
}

/// While statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub id: i64,
    pub condition: Expression,
    pub body: Statement,
    pub src: SourceLocation,
}

/// Modifier definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierDefinition {
    pub id: i64,
    pub name: String,
    #[serde(rename = "virtual")]
    pub r#virtual: bool,
    pub visibility: Visibility,
    pub parameters: ParameterList,
    pub body: Option<Block>,
    pub src: SourceLocation,
    pub scope: Option<i64>,
    pub documentation: Option<Documentation>,
    pub overrides: Option<OverrideSpecifier>,
    #[serde(default)]
    pub nodes: Vec<ModifierDefinitionNode>,
}

/// Modifier invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    Modifier,
    BaseConstructorSpecifier,
    ModifierInvocation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierInvocation {
    pub id: i64,
    pub kind: Option<ModifierInvocationKind>,
    #[serde(rename = "modifierName")]
    pub modifier_name: IdentifierPath,
    #[serde(default)]
    pub arguments: Option<Vec<Expression>>,
    pub src: SourceLocation,
}

/// New expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewExpression {
    pub id: i64,
    #[serde(rename = "typeName")]
    pub type_name: TypeName,
    #[serde(default)]
    pub arguments: Option<Vec<Expression>>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "argumentTypes")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
}

/// Enum definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumDefinition {
    pub id: i64,
    pub name: String,
    pub members: Vec<EnumValue>,
    pub src: SourceLocation,
    pub scope: Option<i64>,
    pub documentation: Option<Documentation>,
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<String>,
    #[serde(default)]
    pub nodes: Vec<EnumDefinitionNode>,
}

/// User defined value type definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDefinedValueTypeDefinition {
    pub id: i64,
    pub name: String,
    pub src: SourceLocation,
    #[serde(default)]
    pub nodes: Vec<UserDefinedValueTypeDefinitionNode>,
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<String>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    #[serde(rename = "underlyingType")]
    pub underlying_type: TypeName,
}

/// Error definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    pub id: i64,
    pub name: String,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    pub scope: Option<i64>,
    pub documentation: Option<Documentation>,
    #[serde(default)]
    pub nodes: Vec<ErrorDefinitionNode>,
}

/// Event definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventDefinition {
    pub id: i64,
    pub name: String,
    pub anonymous: bool,
    #[serde(rename = "eventSelector")]
    pub event_selector: Option<String>,
    pub parameters: ParameterList,
    pub src: SourceLocation,
    pub scope: Option<i64>,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    #[serde(default)]
    pub nodes: Vec<EventDefinitionNode>,
    pub documentation: Option<Documentation>,
}

/// Function type name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionTypeName {
    pub id: i64,
    #[serde(rename = "parameterTypes")]
    pub parameter_types: Vec<TypeName>,
    #[serde(rename = "returnParameterTypes")]
    pub return_parameter_types: Vec<TypeName>,
    pub visibility: String,
    #[serde(rename = "stateMutability")]
    pub state_mutability: String,
    pub src: SourceLocation,
}

/// Struct definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructDefinition {
    pub id: i64,
    pub name: String,
    pub members: Vec<VariableDeclaration>,
    pub src: SourceLocation,
    pub scope: Option<i64>,
    pub documentation: Option<Documentation>,
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<String>,
    #[serde(rename = "usedInEvents")]
    pub used_in_events: Option<bool>,
    #[serde(default)]
    pub nodes: Vec<StructDefinitionNode>,
}

/// Try catch clause.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TryCatchClause {
    pub id: i64,
    pub kind: String,
    #[serde(rename = "errorName")]
    pub error_name: Option<String>,
    pub parameters: Option<ParameterList>,
    pub block: Block,
    pub src: SourceLocation,
}

/// Try statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TryStatement {
    pub id: i64,
    pub expression: Expression,
    #[serde(rename = "returnParameters")]
    pub return_parameters: ParameterList,
    pub clauses: Vec<TryCatchClause>,
    pub src: SourceLocation,
}

// Low complexity

/// Elementary type name expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementaryTypeNameExpression {
    pub id: i64,
    #[serde(rename = "typeName")]
    pub type_name: ElementaryTypeName,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "argumentTypes")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

/// Emit statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmitStatement {
    pub id: i64,
    #[serde(rename = "eventCall")]
    pub event_call: FunctionCall,
    pub src: SourceLocation,
}

/// Inheritance specifier.
/// Inheritance specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InheritanceSpecifier {
    pub id: i64,
    #[serde(rename = "baseName")]
    pub base_name: IdentifierPath,
    #[serde(default)]
    pub arguments: Option<Vec<Expression>>,
    pub src: SourceLocation,
}

/// Inline assembly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineAssembly {
    pub id: i64,
    pub operations: Option<serde_json::Value>,
    #[serde(rename = "externalReferences")]
    pub external_references: Option<Vec<ExternalReference>>,
    pub src: SourceLocation,
    pub documentation: Option<Documentation>,
    #[serde(default)]
    pub flags: Vec<String>,
}

/// Override specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverrideSpecifier {
    pub id: i64,
    #[serde(default)]
    pub overrides: Vec<IdentifierPath>,
    pub src: SourceLocation,
}

/// Parameter list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterList {
    pub id: i64,
    pub parameters: Vec<VariableDeclaration>,
    pub src: SourceLocation,
    #[serde(default)]
    pub nodes: Vec<ParameterListNode>,
}

/// Revert statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevertStatement {
    pub id: i64,
    #[serde(rename = "errorCall")]
    pub error_call: FunctionCall,
    pub src: SourceLocation,
}

/// User defined type name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDefinedTypeName {
    pub id: i64,
    #[serde(rename = "pathNode")]
    pub path_node: Option<IdentifierPath>,
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Using for directive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsingForDirective {
    pub id: i64,
    #[serde(rename = "libraryName")]
    pub library_name: Option<IdentifierPath>,
    #[serde(rename = "typeName")]
    pub type_name: Option<UserDefinedTypeName>,
    pub operations: Option<Vec<String>>,
    pub src: SourceLocation,
    pub global: Option<bool>,
    #[serde(default)]
    pub nodes: Vec<UsingForDirectiveNode>,
    pub scope: Option<i64>,
}

/// Break statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Break {
    pub id: i64,
    pub src: SourceLocation,
}

/// Continue statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Continue {
    pub id: i64,
    pub src: SourceLocation,
}

/// Elementary type name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementaryTypeName {
    pub id: i64,
    pub name: ElementaryType,
    pub src: SourceLocation,
    #[serde(rename = "stateMutability")]
    pub state_mutability: Option<String>,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

/// Enum value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumValue {
    pub id: i64,
    pub name: String,
    #[serde(rename = "nameLocation")]
    pub name_location: String,
    pub src: SourceLocation,
}

/// Identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    pub id: i64,
    pub name: String,
    #[serde(rename = "overloadedDeclarations")]
    #[serde(default)]
    pub overloaded_declarations: Vec<i64>,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "argumentTypes")]
    pub argument_types: Option<Vec<TypeDescriptions>>,
}

/// Identifier path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierPath {
    pub id: i64,
    pub name: String,
    #[serde(rename = "nameLocations")]
    pub name_locations: Option<Vec<String>>,
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: Option<i64>,
    pub src: SourceLocation,
}

/// Import directive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDirective {
    pub id: i64,
    #[serde(rename = "absolutePath")]
    pub absolute_path: String,
    pub file: String,
    #[serde(rename = "unitAlias")]
    pub unit_alias: Option<String>,
    #[serde(rename = "symbolAliases")]
    #[serde(default)]
    pub symbol_aliases: Vec<SymbolAlias>,
    pub scope: Option<i64>,
    #[serde(rename = "sourceUnit")]
    pub source_unit: Option<i64>,
    pub src: SourceLocation,
    #[serde(rename = "nameLocation")]
    pub name_location: Option<String>,
    #[serde(default)]
    pub nodes: Vec<ImportDirectiveNode>,
}

/// Literal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Literal {
    pub id: i64,
    pub kind: LiteralKind,
    pub value: String,
    #[serde(rename = "hexValue")]
    pub hex_value: Option<String>,
    pub subdenomination: Option<String>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "isConstant")]
    pub is_constant: bool,
    #[serde(rename = "isLValue")]
    pub is_l_value: bool,
    #[serde(rename = "isPure")]
    pub is_pure: bool,
    #[serde(rename = "lValueRequested")]
    pub l_value_requested: bool,
    #[serde(rename = "formattedValue")]
    pub formatted_value: Option<String>,
}

/// Placeholder statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderStatement {
    pub id: i64,
    pub src: SourceLocation,
}

/// Pragma directive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PragmaDirective {
    pub id: i64,
    pub literals: Vec<String>,
    pub src: SourceLocation,
    #[serde(default)]
    pub nodes: Vec<PragmaDirectiveNode>,
}

/// Structured documentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredDocumentation {
    pub id: i64,
    pub text: String,
    pub src: SourceLocation,
    pub url: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub notice: Option<String>,
    pub dev: Option<String>,
    #[serde(default)]
    pub params: Vec<StructuredDocumentationParameter>,
    #[serde(default)]
    pub returns: Vec<StructuredDocumentationReturn>,
    #[serde(default)]
    pub custom: Vec<StructuredDocumentationCustom>,
}

/// Function call options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionCallOptions {
    pub id: i64,
    pub expression: Expression,
    pub names: Vec<String>,
    pub options: Vec<Expression>,
    pub src: SourceLocation,
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
    #[serde(rename = "nameLocations")]
    #[serde(default)]
    pub name_locations: Option<Vec<String>>,
}

// ============================================================================
/// Auxiliary Types
// ============================================================================
/// External reference for inline assembly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalReference {
    pub declaration: i64,
    #[serde(rename = "isOffset")]
    pub is_offset: bool,
    #[serde(rename = "isSlot")]
    pub is_slot: bool,
    pub src: SourceLocation,
    #[serde(rename = "valueSize")]
    pub value_size: i64,
}

/// Symbol alias for import directives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolAlias {
    pub foreign: Identifier,
    pub local: Option<String>,
    #[serde(rename = "nameLocation")]
    pub name_location: String,
}

/// Structured documentation parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredDocumentationParameter {
    pub name: String,
    pub description: String,
}

/// Structured documentation return value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredDocumentationReturn {
    pub name: Option<String>,
    pub description: String,
}

/// Structured documentation custom tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredDocumentationCustom {
    pub tag: String,
    pub content: String,
}

// ============================================================================
// Tagged Enums
// ============================================================================

/// Expression nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Expression {
    Assignment(Box<Assignment>),
    BinaryOperation(Box<BinaryOperation>),
    Conditional(Box<Conditional>),
    ElementaryTypeNameExpression(Box<ElementaryTypeNameExpression>),
    FunctionCall(Box<FunctionCall>),
    Identifier(Box<Identifier>),
    IndexAccess(Box<IndexAccess>),
    Literal(Box<Literal>),
    MemberAccess(Box<MemberAccess>),
    NewExpression(Box<NewExpression>),
    TupleExpression(Box<TupleExpression>),
    UnaryOperation(Box<UnaryOperation>),
    VariableDeclarationStatement(Box<VariableDeclarationStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
}

/// Statement nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum Statement {
    Block(Box<Block>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    DoWhileStatement(Box<DoWhileStatement>),
    EmitStatement(Box<EmitStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    ForStatement(Box<ForStatement>),
    IfStatement(Box<IfStatement>),
    InlineAssembly(Box<InlineAssembly>),
    PlaceholderStatement(Box<PlaceholderStatement>),
    Return(Box<Return>),
    RevertStatement(Box<RevertStatement>),
    TryStatement(Box<TryStatement>),
    UncheckedBlock(Box<UncheckedBlock>),
    VariableDeclarationStatement(Box<VariableDeclarationStatement>),
    WhileStatement(Box<WhileStatement>),
}

/// Type name nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum TypeName {
    ArrayTypeName(Box<ArrayTypeName>),
    ElementaryTypeName(ElementaryTypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Box<Mapping>),
    UserDefinedTypeName(UserDefinedTypeName),
}

/// Function call expressions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum FunctionCallExpression {
    ElementaryTypeNameExpression(ElementaryTypeNameExpression),
    FunctionCall(Box<FunctionCall>),
    FunctionCallOptions(FunctionCallOptions),
    Identifier(Identifier),
    MemberAccess(MemberAccess),
    NewExpression(NewExpression),
}

/// Do while statement (not in fixture but needed for Statement enum).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoWhileStatement {
    pub id: i64,
    pub condition: Expression,
    pub body: Statement,
    pub src: SourceLocation,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;
    use serde_json::Value;
    use serde_path_to_error::deserialize;
    use std::fs;
    use walkdir::WalkDir;

    fn find_deserialization_error(content: &str) -> String {
        let value: Value = serde_json::from_str(content).expect("Failed to parse JSON");
        find_error_in_value(&value, "root")
    }

    fn find_error_in_value(value: &Value, path: &str) -> String {
        // Bottom-up approach: check children first, then current node
        // If any child has an error, return it immediately
        if let Some(obj) = value.as_object() {
            // Check all children first (bottom-up)
            for (key, val) in obj {
                let result = find_error_in_value(val, &format!("{}.{}", path, key));
                if !result.is_empty() {
                    return result;
                }
            }

            // If all children pass, try to parse this node
            if let Some(node_type) = obj.get("nodeType") {
                if let Some(type_str) = node_type.as_str() {
                    return try_parse_node(value, path, type_str);
                }
            }
        }

        // Check array elements (bottom-up)
        if let Some(arr) = value.as_array() {
            for (i, item) in arr.iter().enumerate() {
                let result = find_error_in_value(item, &format!("{}[{}]", path, i));
                if !result.is_empty() {
                    return result;
                }
            }
        }

        String::new()
    }

    fn try_parse_node(value: &Value, path: &str, node_type: &str) -> String {
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
                            node_type, path, field_path, err, json_str
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
        for entry in WalkDir::new("fixtures/ast")
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path().extension().map_or(false, |e| e == "json") {
                let content =
                    fs::read_to_string(entry.path()).expect("Failed to read fixture file");
                let result: Result<SourceUnit, _> = serde_json::from_str(&content);
                if let Err(e) = result {
                    let error_msg = find_deserialization_error(&content);
                    panic!(
                        "Failed to parse {:?}: {}\nError details:\n{}",
                        entry.path(),
                        e,
                        error_msg
                    );
                }
            }
        }
    }
}
