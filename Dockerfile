FROM rust:1.92-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin mcp-server

FROM scratch
COPY --from=builder /app/target/release/mcp-server /mcp-server
ENTRYPOINT ["/mcp-server"]
