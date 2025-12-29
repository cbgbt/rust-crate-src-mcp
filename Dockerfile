FROM --platform=$BUILDPLATFORM rust:1.92-slim AS builder
ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y musl-tools gcc-aarch64-linux-gnu
RUN case "$TARGETPLATFORM" in \
      "linux/amd64") echo "x86_64-unknown-linux-musl" > /target.txt ;; \
      "linux/arm64") echo "aarch64-unknown-linux-musl" > /target.txt ;; \
      *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add $(cat /target.txt)

WORKDIR /app
COPY . .
RUN if [ "$(cat /target.txt)" = "aarch64-unknown-linux-musl" ]; then \
      export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc; \
    fi && \
    cargo build --release --bin rust-crate-src-mcp --target $(cat /target.txt) && \
    cp target/$(cat /target.txt)/release/rust-crate-src-mcp /rust-crate-src-mcp

FROM scratch
COPY --from=builder /rust-crate-src-mcp /rust-crate-src-mcp
ENTRYPOINT ["/rust-crate-src-mcp"]
