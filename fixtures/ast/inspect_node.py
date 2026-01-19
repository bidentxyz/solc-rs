#!/usr/bin/env python3
"""
Solidity AST Node Inspector

This script analyzes Solidity AST JSON files to identify all node types
and their children, sorted by number of children for bottom-up implementation.

Usage:
    python fixtures/ast/inspect_node.py [directory]

Default directory: fixtures/ast
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

# Common fields that contain AST nodes in Solidity compiler output
AST_NODE_FIELDS = {
    "nodes",
    "foreign",
    "body",
    "arguments",
    "declarations",
    "nodes",
    "foreign",
    "body",
    "arguments",
    "declarations",
    "baseContracts",
    "functions",
    "events",
    "modifiers",
    "variables",
    "parameters",
    "returnParameters",
    "condition",
    "trueBody",
    "falseBody",
    "initialization",
    "value",
    "assignment",
    "leftHandSide",
    "rightHandSide",
    "expression",
    "vReturnValue",
    "vFunctionCall",
    "vTryCall",
    "statements",
    "documentation",
    "typeName",
    "type",
    "element",
    "memberName",
    "members",
    "attributes",
    "arguments",
    "components",
    "arrayExpression",
    "indexExpression",
    "base",
    "expressionName",
    "memberExpression",
    "newExpression",
    "expressionType",
    "superFunction",
    "constructor",
    "fallbackReceive",
    "receiveEther",
    "fallback",
    "symbolAliases",
    "originalName",
    "nameLocation",
    "functionSelector",
}


def is_ast_node(value):
    """Check if a value is an AST node (dict with nodeType field)"""
    return isinstance(value, dict) and "nodeType" in value


def extract_ast_nodes(value):
    """
    Recursively extract all AST nodes from a value.

    Returns:
        List of node dictionaries
    """
    nodes = []

    if is_ast_node(value):
        nodes.append(value)

    if isinstance(value, dict):
        for key, val in value.items():
            # Check if this key is likely to contain AST nodes
            if key in AST_NODE_FIELDS or is_ast_node(val):
                nodes.extend(extract_ast_nodes(val))
            elif isinstance(val, (dict, list)):
                nodes.extend(extract_ast_nodes(val))
    elif isinstance(value, list):
        for item in value:
            if is_ast_node(item):
                nodes.extend(extract_ast_nodes(item))
            elif isinstance(item, (dict, list)):
                nodes.extend(extract_ast_nodes(item))

    return nodes


def analyze_directory(directory):
    """
    Analyze all JSON files in the given directory.

    Args:
        directory: Path to directory containing JSON AST files

    Returns:
        Tuple of (node_types dict, all_node_types set)
        - node_types: dict mapping node_type -> set of child node types
        - all_node_types: set of all node types found
    """
    node_types = defaultdict(set)  # nodeType -> set of child nodeTypes
    all_node_types = set()  # Track all node types found

    directory = Path(directory)
    json_files = list(directory.glob("*.json"))

    print(f"📁 Scanning {len(json_files)} JSON files in {directory}/\n")

    for json_file in json_files:
        try:
            with open(json_file, "r") as f:
                data = json.load(f)

            # Extract all AST nodes from the file
            all_nodes = extract_ast_nodes(data)

            # For each node, find its children
            for node in all_nodes:
                node_type = node.get("nodeType", "Unknown")
                all_node_types.add(node_type)

                # Find child nodes in common fields
                for field in AST_NODE_FIELDS:
                    if field in node:
                        value = node[field]

                        if is_ast_node(value):
                            # Single child node
                            child_nodes = extract_ast_nodes(value)
                            for child_node in child_nodes:
                                child_type = child_node.get("nodeType", "Unknown")
                                node_types[node_type].add(child_type)
                        elif isinstance(value, list):
                            # Array of child nodes
                            for item in value:
                                if is_ast_node(item):
                                    child_nodes = extract_ast_nodes(item)
                                    for child_node in child_nodes:
                                        child_type = child_node.get(
                                            "nodeType", "Unknown"
                                        )
                                        node_types[node_type].add(child_type)

        except Exception as e:
            print(f"⚠️  Error processing {json_file}: {e}", file=sys.stderr)

    return node_types, all_node_types


def print_results(node_types, all_node_types):
    """
    Print the analysis results sorted by number of children.

    Args:
        node_types: dict mapping node_type -> set of child node types
        all_node_types: set of all node types found
    """
    # Add missing node types (those with no children)
    for node_type in all_node_types:
        if node_type not in node_types:
            node_types[node_type] = set()

    # Sort by number of children (ascending for bottom-up approach)
    sorted_types = sorted(node_types.items(), key=lambda x: len(x[1]))

    print(f"📊 Found {len(sorted_types)} unique node types\n")
    print("=" * 70)

    for node_type, children in sorted_types:
        child_count = len(children)

        if child_count == 0:
            print(f"\n🎯 {node_type}: NO CHILDREN (START HERE!)")
            print("-" * 70)
        elif child_count <= 2:
            print(f"\n{node_type}: {child_count} child")
            print(f"   {sorted(children)}")
            print("-" * 70)
        elif child_count <= 5:
            print(f"\n{node_type}: {child_count} children")
            for child in sorted(children):
                print(f"   • {child}")
            print("-" * 70)
        elif child_count <= 10:
            print(f"\n{node_type}: {child_count} children")
            print(f"   {', '.join(sorted(children))}")
            print("-" * 70)
        else:
            print(f"\n{node_type}: {child_count} children")
            print(f"   {', '.join(sorted(children))}")
            print("-" * 70)

    print("\n" + "=" * 70)
    print(f"\n✅ Total: {len(sorted_types)} node types")
    print(f"   • {sum(1 for _, c in sorted_types if len(c) == 0)} with no children")
    print(
        f"   • {sum(1 for _, c in sorted_types if 0 < len(c) <= 5)} with 1-5 children"
    )
    print(
        f"   • {sum(1 for _, c in sorted_types if 5 < len(c) <= 20)} with 6-20 children"
    )


def main():
    """Main entry point."""
    # Get directory from command line or use default
    directory = sys.argv[1] if len(sys.argv) > 1 else "fixtures/ast"

    # Ensure directory exists
    if not Path(directory).exists():
        print(f"❌ Error: Directory '{directory}' not found", file=sys.stderr)
        sys.exit(1)

    # Analyze the directory
    node_types, all_node_types = analyze_directory(directory)

    if not all_node_types:
        print("❌ No AST nodes found in the JSON files", file=sys.stderr)
        sys.exit(1)

    # Print results
    print_results(node_types, all_node_types)


if __name__ == "__main__":
    main()
