# rust-crate-src

Fetch Rust crate source code from crates.io. Available as a library and MCP server.

## Usage

### As a Library

```rust
use rust_crate_src::get_crate_source;

let source = get_crate_source("serde", None).await?;
println!("Extracted to: {}", source.checkout_path.display());

// With version constraint
let source = get_crate_source("tokio", Some("^1.0")).await?;
```

### As an MCP Server

```bash
cargo run --bin mcp-server
```

Exposes the `get_rust_crate_source` tool:
- **crate_name** (required): Name of the crate to fetch
- **version** (optional): Semver version constraint (e.g., "1.0", "^1.2", "~1.2.3")

Returns:
- **crate_name**: Echo of requested crate
- **version**: Resolved exact version
- **checkout_path**: Path to extracted source
- **message**: Human-readable summary

## Development

```bash
make check    # Run all checks (test + lint + format) - REQUIRED before commits
make format   # Auto-format code
make test     # Run unit tests
make integ    # Run integration tests (requires network)
```

## Architecture

| Module | Purpose |
|--------|--------|
| `lib.rs` | Public API facade |
| `version.rs` | Version resolution via crates.io API |
| `extract.rs` | Download and extract .crate tarballs |
| `bin/mcp-server.rs` | MCP server using rmcp |

## For AI Agents

See [AGENTS.md](AGENTS.md) for development guidelines.
