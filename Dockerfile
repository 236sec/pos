# Builder stage
FROM rust:1.85 AS builder
WORKDIR /app
ARG SQLX_OFFLINE=true
ENV SQLX_OFFLINE=true

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pos /usr/local/bin/pos
EXPOSE 8000
CMD ["pos"]
