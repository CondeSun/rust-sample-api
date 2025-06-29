# Rust Project Structure

From 

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

User Repo can be switched out during tests

```rust
#[cfg(test)]
mod tests {
use super::*;

struct MockRepo;
    impl UserRepo for MockRepo {
        fn find_by_id(&self, _: uuid::Uuid) -> Result<User, AppError> {
            Ok(User { id: uuid::Uuid::new_v4(), email: "test@example.com".into(), is_active: true })
        }
    }
    #[test]
    fn returns_user_if_active() {
        let service = UserService::new(MockRepo {});
        assert!(service.get_user(uuid::Uuid::new_v4()).is_ok());
    }
}
```
