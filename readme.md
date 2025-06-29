# Rust Project Structure
Typicall Rust Project Structure
```
/my_app
├── Cargo.toml
└── src
    ├── main.rs                --> Entry point
    ├── lib.rs                 --> App-wide exports and setup
    ├── config/                --> Config parsing and validation
    ├── api/                   --> HTTP handlers or CLI commands
    ├── services/              --> Business logic
    ├── domain/                --> Core types, enums, interfaces
    ├── db/                    --> Persistence layer (Postgres, Redis etc.)
    ├── errors/                --> Shared error types
    ├── utils/                 --> Generic helpers
```