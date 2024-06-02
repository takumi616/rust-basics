# Create a stage for building the application.
FROM rust:1.78.0-slim-bullseye AS build

WORKDIR /app

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/rust-basics /bin/server
EOF

# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. 
FROM debian:bullseye-slim AS final

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# What the container should run when it is started.
CMD ["/bin/server"]