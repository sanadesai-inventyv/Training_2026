# Rust Serialization & Deserialization (Serde Practice)

This project demonstrates how to use **Serde** in Rust to perform:

- Serialization (Struct → JSON)
- Deserialization (JSON → Struct)

Both **direct method** and **raw JSON method** are practiced using a complex structure.

This is created for learning Rust structures and Serde concepts.

---


## 📦 Dependencies

Added these in `Cargo.toml`:

```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

