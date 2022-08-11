FROM rust:1.62.1-slim-buster as builder
WORKDIR /api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as production
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/gratitude_notes_api /usr/local/bin/gratitude_notes_api
EXPOSE 8080
CMD ["gratitude_notes_api"]