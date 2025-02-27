# build
FROM rust:1.84 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# prod
FROM debian:bookworm-slim AS runner
RUN apt update && apt install libpq5 -y
COPY --from=builder /usr/local/cargo/bin/app /usr/local/cargo/bin/synth /usr/local/bin/
CMD synth && app
