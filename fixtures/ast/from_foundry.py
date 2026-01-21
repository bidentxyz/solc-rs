#!/usr/bin/env python3
"""
Extract AST data from Foundry artifacts.

This script reads all JSON files from a Foundry artifacts directory,
extracts the 'ast' key from each file, and writes it to the output directory.

Usage:
    python3 from_foundry.py [artifacts-dir] [output-dir]

Example:
    python3 from_foundry.py /home/pyk/codearena/2025-10-covenant/out fixtures/ast/covenant
"""

import json
import sys
from pathlib import Path


def main():
    # Validate command line arguments
    if len(sys.argv) != 3:
        print("Usage: python3 from_foundry.py [artifacts-dir] [output-dir]")
        print("Example: python3 from_foundry.py /path/to/artifacts /path/to/output")
        sys.exit(1)

    artifacts_dir = Path(sys.argv[1])
    output_dir = Path(sys.argv[2])

    # Validate artifacts directory
    if not artifacts_dir.exists():
        print(f"Error: Artifacts directory '{artifacts_dir}' does not exist")
        sys.exit(1)

    if not artifacts_dir.is_dir():
        print(f"Error: '{artifacts_dir}' is not a directory")
        sys.exit(1)

    # Create output directory if it doesn't exist
    output_dir.mkdir(parents=True, exist_ok=True)

    # Find all JSON files recursively
    json_files = list(artifacts_dir.glob("**/*.json"))

    if not json_files:
        print(f"Warning: No JSON files found in '{artifacts_dir}'")
        return

    print(f"Found {len(json_files)} JSON files in '{artifacts_dir}'")

    processed_count = 0
    skipped_count = 0

    for json_file in json_files:
        try:
            with open(json_file, "r") as f:
                data = json.load(f)

            # Check if 'ast' key exists
            if "ast" not in data:
                skipped_count += 1
                continue

            # Prepare output file path using the original filename
            output_file = output_dir / json_file.name

            # Write the AST content
            with open(output_file, "w") as f:
                json.dump(data["ast"], f, indent=2)

            processed_count += 1
            print(f"Processed: {json_file.name} -> {output_file}")

        except json.JSONDecodeError as e:
            print(f"Error parsing {json_file}: {e}")
            skipped_count += 1
        except Exception as e:
            print(f"Error processing {json_file}: {e}")
            skipped_count += 1

    print(
        f"\nSummary: {processed_count} files processed, {skipped_count} files skipped"
    )


if __name__ == "__main__":
    main()
