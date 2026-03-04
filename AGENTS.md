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

## rust-lint

Use `rust-lint` to make sure code follow the style guidelines.

```shell
# Lint all rust code
rust-lint
```

# Guidelines

- [README Writing Guidelines](.agents/guidelines/readme.md)
- [Rust Coding Guidelines: Documentation](.agents/guidelines/rust.md)

# Instructions

- [Research Instruction](.agents/instructions/research.md)
- [Create Plan Instruction](.agents/instructions/create-plan.md)
- [Create Git Commit Message Instruction](.agents/instructions/create-git-commit-message.md)
- [Changelog Instruction](.agents/instructions/changelog.md)

# Agent Modes

## Planning Mode

When creating or updating a plan, strictly follow the two-phase process defined
in the **Create Plan Instruction**:

1. **Phase 1: Interrogation**
    - Read the task description.
    - Follow the "Interrogation" phase instructions to ask questions and clarify
      scope.
    - Do not proceed to generation until requirements are fully defined.
2. **Phase 2: Generation**
    - Read `README.md` to understand the project.
    - Follow the "Generation" phase instructions to create a new plan.
    - **IMPORTANT**: Ensure the plan includes the base success criteria:
        - `rust-lint` passes
        - `cargo clippy -- -D warnings` passes
        - `cargo build` succeeds
        - `cargo test` passes
3. **Review & Iterate**
    - If the user provides feedback on the generated plan, update the file
      accordingly.

## Building Mode

When implementing a plan:

1. Update the plan status as in progress.
2. Read README.md to understand the project.
3. Use the guidelines.
4. Use the `cargo-txt` tool.
5. Use the thinking tool.
6. **IMPORTANT**: Do not use git restore commands (can cause data loss).
7. **IMPORTANT**: Review and update the plan checklist after implementation.

## Git Commit Mode

When writing Git Commit message:

1. Read README.md to understand the project.
2. Use the guidelines.
3. Use the thinking tool.
4. Read & follow **Create Git Commit Message Instruction**.

## Research Mode

When creating or updating research documentation:

1. Read **Research Instruction** to understand the format and structure.
2. Read all sources to get complete context.
3. Create or update the file in `.agents/research/` directory using
   dash-separated lowercase filename format (e.g., `tokio-json-rpc.md`).
4. Follow the document format structure:
    - Title with descriptive topic name
    - Sources section with all source links
    - Answered Questions section with numbered list of all questions
    - Numbered sections for each question with descriptive headers
    - `**Answering**: ` references at the top of each question section
5. Include section separators (`---`) between major sections.
6. Review to ensure no extra content is included.

## Changelog Mode

When creating or updating the changelog:

1. Read **Changelog Instruction** to understand the format and process.
2. Check if `CHANGELOG.md` exists in the project root.
3. Use git history to understand changes:
    - If changelog exists: `git log v0.2.0...HEAD --oneline` (where v0.2.0 is
      the previous version git tag)
    - Otherwise: `git log --oneline`
4. Review individual commits: `git show <commit-hash>`
5. Create new changelog or add new version entry following Keep a Changelog
   1.1.0.
6. **IMPORTANT**: Do not edit previous changelog entries - preserve history.
7. Categorize changes: Added, Changed, Deprecated, Removed, Fixed, Security.
8. Verify all commits are accounted for in the changelog.
