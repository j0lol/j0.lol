FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    --mount=type=cache,target=static \
    cargo build --release && mv ./target/release/website ./website

# Runtime image
FROM debian:bookworm-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

# USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/posts /app/posts
COPY --from=builder /usr/src/app/static /app/static
COPY --from=builder /usr/src/app/website /app/website

ENV SITE_ENV=PROD
# Run the app
CMD ./website
