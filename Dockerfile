FROM --platform=$BUILDPLATFORM rust:1.92-slim AS builder
ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y musl-tools
RUN case "$TARGETPLATFORM" in \
      "linux/amd64") echo "x86_64-unknown-linux-musl" > /target.txt ;; \
      "linux/arm64") echo "aarch64-unknown-linux-musl" > /target.txt ;; \
      *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add $(cat /target.txt)

WORKDIR /app
COPY . .
RUN cargo build --release --bin rust-crate-src-mcp --target $(cat /target.txt) && \
    cp target/$(cat /target.txt)/release/rust-crate-src-mcp /rust-crate-src-mcp

FROM scratch
COPY --from=builder /rust-crate-src-mcp /rust-crate-src-mcp
ENTRYPOINT ["/rust-crate-src-mcp"]
