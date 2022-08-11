# Builds the minified production executable 

FROM rust:1.62.1-slim-buster as builder
WORKDIR /
# Generate a new crate directory at /api.
RUN cargo new api
WORKDIR /api
# Build just the dependencies.
COPY /Cargo.toml /Cargo.lock ./
RUN cargo build --release --locked
# Copy everything else
COPY . .
# Make sure to run `touch` so Cargo rebuilds my code.
RUN touch src/main.rs && cargo build --release --locked


# As the executable is only required to run the web server, copying it from the rust image
# into debain can help reduce the overall image size.
FROM debian:buster-slim
COPY --from=builder /api/target/release/gratitude_notes_api /usr/local/bin/gratitude_notes_api
# API is mapped to port 8080 (Should later change this to ENV var)
EXPOSE 8080
# Runs production server
CMD ["gratitude_notes_api"]