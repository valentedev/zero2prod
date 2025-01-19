# FROM lukemathwalker/cargo-chef:latest-rust-1.81 AS chef
FROM rust:1.81 AS chef
RUN cargo install cargo-chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# 1 - Compute the "recipe" file. 
FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# 2 - Caches our dependencies and and build the binary. 
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release --bin zero2prod

#  3 - Runtime Environment
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./zero2prod"]