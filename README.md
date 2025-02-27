# ARound -- Code Review

Docker 27.4 | Nix 2.24 | Rust 1.84 , Postgres 17.4 , Diesel 2.2

Assuming dev tools like `jq` and `curl` installed.

### Deployment

Docker (easy + demo):
- `cp .env.local .env` for envars
- `docker compose up` for startup
- `docker container exec -ti around-db-1 psql` for manual database inspection

Nix (cool + dev):
- `direnv allow` to enjoy everything in shell
- `pg_ctl init` to initialize Postgres
- `pg_ctl start` to start Postgres
- `diesel migration run` for migrations
- `cargo r --bin synth` for synthetic data generation
- `psql` for manual database inspection
- `cargo r --bin app` for server startup

Manual (painful + untested):
- Rust from rustup should be enough
- install Postgres with `apt install postgresql`
- install Diesel with `cargo install diesel_cli --features postgres`
- set up shell envars as listed in `flake.nix` or pass them everywhere manually
- do everything as in Nix except first bullet

### Testing

Please give me a few more hours for deploy, test suite, and OpenAPI UI. For now just use curl:
- `curl -X GET localhost:8080/v0/docs | jq .` for OpenAPI spec
- `curl -X GET localhost:8080/v0/foo` for nonexistent path
- `curl -X GET localhost:8080/v0/users/1` for missing user
- `curl -X GET localhost:8080/v0/users/foo` for malformatted parameter
- `curl -X GET localhost:8080/v0/users/697020080` for existing user
- `curl -X POST localhost:8080/v0/users -H "Content-Type: application/json" -d '{"data":{"id":1,"name":"foo","fingerprint":"asldfjk"}}'` for user creation (try it twice in a row)
- `curl -X POST localhost:8080/v0/users -H "Content-Type: application/json" -d '{"data":{"id":"foo","name":"foo","fingerprint":"asldfjk"}}'` for malformatted payload
- `curl -X POST localhost:8080/v0/users -H "Content-Type: application/json" -d '{"data":{"id"1,"name":"foo","fingerprint":"asldfjk"}}'` for malformatted payload
- `curl -X PUT localhost:8080/v0/users -H "Content-Type: application/json" -d '{"data":{"id":1,"name":"bar","fingerprint":"asldfjk"}}'` for user update (try it twice in a row)
- `curl -X DELETE localhost:8080/v0/users/1` for missing user
- `curl -X DELETE localhost:8080/v0/users/697020080` for existing user (try it twice in a row)
- `curl -X GET "localhost:8080/v0/events?limit=1"` for events
- `curl -X GET localhost:8080/v0/events` for events
- `curl -X GET "localhost:8080/v0/events?limit=16&offset=32&start=2025-07-01T00:00:00Z"` for events
- `curl -X GET "localhost:8080/v0/events?limit=foo"` for malformatted query

### Justifications

Code:
- axum because never worked with it
- would gladly avoid Chrono but SystemTime has no Dummy instance

Storage:
- not Duckdb because multiple clients
- not Redis because range queries
- not plain file because ORM 
