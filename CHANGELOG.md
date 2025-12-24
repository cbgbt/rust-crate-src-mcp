# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/cbgbt/rust-crate-src-mcp/releases/tag/v0.1.0) - 2025-12-24

### Added

- add GitHub Actions CI workflows
- add minimal scratch-based Dockerfile
- add cargo-deny for license and duplicate checking
- expand tool description with use cases
- wire up lib facade and MCP server
- implement crate extraction
- implement version resolution

### Fixed

- support GNU Make < 4.4
- rename
- use rustls-only TLS, remove openssl dependency

### Other

- fix rust style nits
- add dockerignore
- *(deny)* disallow wildcard deps
- rename ot rust-crate-src-mcp
- format code with cargo fmt
- add license and symposium attribution
- add Makefile, AGENTS.md, README.md, and LOC lint
- add MCP integration tests
- interfaces and types for rust-crate-src
