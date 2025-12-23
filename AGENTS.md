# AGENTS.md

Development guide for agents working on this codebase.

**Before committing, always run `make check`.**

## Quick Reference

```bash
make format   # Auto-format all code
make check    # Run all checks (test + lint + format-check) - ALWAYS RUN BEFORE COMMITTING
make integ    # Run integration tests (requires network)
```

If you must use cargo, run ALL cargo commands with '-q' for quiet -- only emits tokens on error/warning.

## Project Structure

```
src/
├── lib.rs           # Public API: get_crate_source()
├── version.rs       # Version resolution against crates.io API
├── extract.rs       # Download & extract .crate files
└── bin/
    └── mcp-server.rs  # MCP server exposing get_rust_crate_source tool
tests/
└── integration.rs   # MCP client integration tests (#[ignore])
```

### Module Responsibilities

- **lib.rs**: Public facade exposing `get_crate_source()` and `CrateSource` type
- **version.rs**: Queries crates.io API, resolves semver constraints to exact versions
- **extract.rs**: Downloads .crate tarballs, extracts to ~/.cargo/registry/src/ cache
- **mcp-server.rs**: Thin MCP wrapper using rmcp, exposes `get_rust_crate_source` tool

### File Size Lint

Modules should stay under 500 lines.
This is checked by `make check`.
Overrides can be added to scripts/lint_loc.py with justification.

## Code Style

- See `~/marvin/docs/style/rust-design.md` when creating new types or modules
- See `~/marvin/docs/style/rust-impl.md` when implementing functions
- snafu for errors (with `#[snafu(module)]`), bon for builders
- Conventional commits: `feat: description` or `fix: description`

## Error Handling

- Each module has its own error type (e.g., `ResolveVersionError`, `ExtractError`)
- Use snafu context selectors imported inside functions: `use error_name::*;`
- Display messages describe what failed, not how to fix it
