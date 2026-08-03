# CLI

## cargo-txt

Use `cargo txt` to access the crate documentation.

The workflow is:

1. Build documentation: `cargo txt build <crate>`
2. List all items: `cargo txt list <lib_name>`
3. View specific item: `cargo txt show <lib_name>::<item>`

For example:

```shell
# Build the serde crate documentation
cargo txt build serde

# List all items in serde
cargo txt list serde

# View serde crate overview
cargo txt show serde

# View serde::Deserialize trait documentation
cargo txt show serde::Deserialize
```
