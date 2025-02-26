# ARound -- Code Review

Rust 1.84 | Postgres 17.4 | Diesel 2.2

### Justifications

Auth:
- assuming QR containing valid Bearer JWT

Storage:
- not Duckdb because multiple clients
- not Redis because range queries
- not plain file because ORM 

Framework:
- axum because never worked with it
