# Solidity AST Files

This directory contains Solidity AST JSON files and tools for analyzing them.

## inspect_node.py

A Python 3 script that analyzes Solidity AST JSON files to identify all node
types and their children, sorted by number of children for bottom-up
implementation.

### Usage

```bash
# Default usage (analyzes fixtures/ast/*.json)
python3 fixtures/ast/inspect_node.py

# Analyze a specific directory
python3 fixtures/ast/inspect_node.py path/to/ast/files
```

### Output

The script outputs a sorted list of all node types found in the JSON files,
organized by the number of children they contain:

```
📁 Scanning 9 JSON files in fixtures/ast/

📊 Found 43 unique node types

======================================================================

🎯 ElementaryTypeName: NO CHILDREN (START HERE!)
----------------------------------------------------------------------
...
```

### Implementation Strategy

The output is sorted in ascending order of child node count, making it easy to
follow a **bottom-up implementation approach**:

1. **Start with nodes that have no children** (marked with 🎯)
    - `ElementaryTypeName`
    - `Identifier`
    - `Literal`
    - `PragmaDirective`
    - And 13 more...

2. **Progress to nodes with few children**
    - Nodes with 1-2 children can be implemented once their dependencies are
      done
    - Example: `ErrorDefinition` depends only on `ParameterList`

3. **Build up to complex nodes**
    - `SourceUnit` has 42 children and should be implemented last
    - `ContractDefinition` has 39 children
    - `FunctionDefinition` has 30 children

### Statistics Example

```
✅ Total: 43 node types
   • 17 with no children
   • 10 with 1-5 children
   • 11 with 6-20 children
   • 5 with 20+ children
```

## AST Files

This directory contains example AST JSON files from various Solidity projects:

- `covenant.json` - Main Covenant contract
- `covenant-chainlink-oracle.json` - Chainlink oracle integration
- `covenant-cross-adapter.json` - Cross-adapter functionality
- `covenant-curator.json` - Curator contract
- `covenant-data-provider.json` - Data provider
- `covenant-latent-swap-lex.json` - Latent swap LEX
- `covenant-no-delegate-call.json` - No delegate call variant
- `covenant-pyth-oracle.json` - Pyth oracle integration
- `covenant-synth-token.json` - Synthetic token

## Node Type Categories

### Terminal Nodes (No Children)

Simple leaf nodes that don't contain other AST nodes:

- `ElementaryTypeName`, `Identifier`, `Literal`
- `PragmaDirective`, `ImportDirective`
- `UserDefinedTypeName`, `IdentifierPath`
- `Mapping`, `ArrayTypeName`
- And more...

### Expression Nodes

Nodes that represent expressions and operations:

- `BinaryOperation`, `UnaryOperation`
- `FunctionCall`, `MemberAccess`, `IndexAccess`
- `Conditional`, `TupleExpression`
- `NewExpression`

### Statement Nodes

Nodes that represent statements:

- `ExpressionStatement`, `VariableDeclarationStatement`
- `IfStatement`, `ForStatement`, `Return`
- `EmitStatement`, `RevertStatement`
- `PlaceholderStatement`

### Declaration Nodes

Nodes that declare new entities:

- `ContractDefinition`, `FunctionDefinition`
- `VariableDeclaration`, `EventDefinition`
- `ErrorDefinition`, `ModifierDefinition`

### Type-Related Nodes

Nodes related to type systems:

- `ElementaryTypeNameExpression`
- `UsingForDirective`, `InheritanceSpecifier`, `OverrideSpecifier`

### Structural Nodes

Nodes that provide structure:

- `SourceUnit` - Root of the AST
- `Block`, `UncheckedBlock`
- `ParameterList`

## Notes

- The script recursively traverses all JSON files and identifies AST nodes by
  looking for objects with a `nodeType` field
- Child relationships are determined by analyzing common AST node fields like
  `nodes`, `body`, `arguments`, etc.
- The analysis is conservative - if a field might contain AST nodes, it's
  included in the analysis
- Some node types may have children that don't appear in the sample JSON files
