# Tools

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

# Guidelines

**CRITICAL**: You must follow the guidelines strictly.

1. Read `.zed/agent/guidelines/readme.md` to understand README guidelines.
2. Read `.zed/agent/guidelines/rust.md` to understand the Rust guidelines.

# Planning Mode

When creating or updating a plan:

1. Read README.md to understand the project.
2. Use the thinking tool.
3. Use `cargo-txt` if needed.
4. Include README.md updates in the plan.
5. **IMPORTANT**: Every plan's Success Criteria section MUST include these base
   criteria:
    - `rust-lint` passes
    - `cargo clippy -- -D warnings` passes
    - `cargo build` succeeds
    - `cargo test` passes
6. Read & follow instructions in `.zed/agent/instructions/create-plan.md`.

# Building Mode

When implementing a plan:

1. Update the plan status as in progress.
2. Read README.md to understand the project.
3. Use the thinking tool.
4. **IMPORTANT**: Do not use git restore commands (can cause data loss).
5. **IMPORTANT**: Review and update the plan checklist after implementation.

# Reviewing Mode

When reviewing staged changes:

1. Read README.md to understand the project.
2. Use the thinking tool.
3. Follow the instructions in `.zed/agent/instructions/review-changes.md`.
4. Use `cargo test` to run tests and `rust-lint` to verify changes.

# Git Commit Mode

When writing Git Commit message:

1. Read README.md to understand the project.
2. Use the thinking tool.
3. Follow instructions in
   `.zed/agent/instructions/create-git-commit-message.md`.
