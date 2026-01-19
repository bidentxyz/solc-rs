#!/usr/bin/env python3
"""
Script to count and group objects by nodeType in JSON files.
Recursively traverses JSON files in fixtures/ast/ and counts all objects
that have a 'nodeType' field.
"""

import json
from collections import defaultdict
from pathlib import Path


def count_node_types(data, counts):
    """
    Recursively traverse a data structure and count objects with nodeType field.

    Args:
        data: The data structure to traverse (dict or list)
        counts: Dictionary to accumulate counts
    """
    if isinstance(data, dict):
        # Check if this dict has a nodeType field
        if "nodeType" in data:
            node_type = data["nodeType"]
            counts[node_type] += 1

        # Recursively traverse all values in the dict
        for value in data.values():
            count_node_types(value, counts)

    elif isinstance(data, list):
        # Recursively traverse all items in the list
        for item in data:
            count_node_types(item, counts)


def main():
    """Main function to process JSON files and display results."""
    ast_dir = Path(__file__).parent
    counts = defaultdict(int)

    # Find all JSON files in the ast directory
    json_files = list(ast_dir.glob("*.json"))

    if not json_files:
        print("No JSON files found in:", ast_dir)
        return

    print(f"Processing {len(json_files)} JSON file(s)...")

    # Process each JSON file
    for json_file in json_files:
        try:
            with open(json_file, "r", encoding="utf-8") as f:
                data = json.load(f)
                count_node_types(data, counts)
                print(f"  Processed: {json_file.name}")
        except json.JSONDecodeError as e:
            print(f"  Error parsing {json_file.name}: {e}")
        except Exception as e:
            print(f"  Error processing {json_file.name}: {e}")

    # Display results in sorted order
    print(f"\nFound {len(counts)} unique node types:")
    print("-" * 50)

    total_count = 0
    for node_type, count in sorted(counts.items(), key=lambda x: x[1], reverse=True):
        print(f"{count:6d} : {node_type}")
        total_count += count

    print("-" * 50)
    print(f"Total: {total_count} nodes")


if __name__ == "__main__":
    main()
