FROM rust:1.92-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin rust-crate-src-mcp

FROM scratch
COPY --from=builder /app/target/release/rust-crate-src-mcp /rust-crate-src-mcp
ENTRYPOINT ["/rust-crate-src-mcp"]
