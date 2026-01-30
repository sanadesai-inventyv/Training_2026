# 🦀 Rust Server Request Tracker (Mutex & RwLock)

This project demonstrates how to build a simple server request tracker in Rust using:

- Enums
- Pattern Matching
- Static Global Variables
- `lazy_static`
- `Mutex`
- `RwLock`

The program counts how many requests are processed and returns messages based on request type.

---
 

## 📦 Dependencies

Add this to your `Cargo.toml`:

```toml
[dependencies]
lazy_static = "1.4"
```

What is Mutex?

- Mutex means Mutual Exclusion.

- Only one thread can access the data at a time.

- Used when both reading and writing happen frequently

What is RwLock?

- RwLock = Read–Write Lock

Allows:
 Multiple readers

Better for reading-heavy applications.

---

