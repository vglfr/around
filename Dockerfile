# build
FROM rust:1.84 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# prod
FROM debian:bookworm-slim AS runner
RUN apt update && apt install libpq5 -y # ca-certificates && update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app
CMD ["app"]
