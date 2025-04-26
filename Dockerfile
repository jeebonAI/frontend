# Stage 1: Generate a recipe file for dependencies
FROM rust:1.86-slim-bullseye AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build dependencies and application
FROM rust:1.86-slim-bullseye AS builder
WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev nodejs npm && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml to extract wasm-bindgen version
COPY Cargo.toml .

# Install cargo-chef and Dioxus CLI
RUN cargo install cargo-chef --locked && \
    cargo install dioxus-cli@0.6.0 --locked

# Build dependencies - this is the key step that will be cached
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code and build the application
COPY . .
RUN dx build --release

# Stage 3: Create a lightweight runtime image with Caddy (optimized for WASM)
FROM caddy:2-alpine

# Copy the built assets from the builder stage
COPY --from=builder /app/dist /usr/share/caddy
COPY --from=builder /app/assets/sw.js /usr/share/caddy/sw.js

# Copy Caddyfile configuration
COPY docker/Caddyfile /etc/caddy/Caddyfile

EXPOSE 80

# Caddy runs automatically
