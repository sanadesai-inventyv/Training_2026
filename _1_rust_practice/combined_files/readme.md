# Combined Files – Rust Practice Project

- Modules & file structure
- Ownership & borrowing
- Structs & implementations
- Mutex and RwLock (concurrency)
- Basic Serde usage
- Loops and control flow

Each topic is implemented in its own module for clarity.

---

## 📁 Project Structure

combined_files/

└── src/

├── main.rs

└── modules/

├── mod.rs

├── loops.rs

├── mutex.rs

├── ownership.rs

├── rwlock.rs

├── serde.rs

└── structures.rs


---

##  Module Overview

### `loops.rs`
Contains examples of Rust loops and basic flow control.

### `ownership.rs`
Demonstrates Rust ownership, borrowing, and mutable references using structs.

### `structures.rs`
Shows how to create structs, implement methods, getters/setters, and basic object-style logic.

### `mutex.rs`
Implements a simple request tracker using `Mutex` for thread-safe shared state.

### `rwlock.rs`
Same idea as `mutex.rs`, but using `RwLock` to allow multiple readers or one writer.

### `serde.rs`
Basic serialization/deserialization example using Serde.

### `mod.rs`
Registers all modules:

```rust
pub mod loops;
pub mod mutex;
pub mod ownership;
pub mod rwlock;
pub mod serde;
pub mod structures;
```
---

`main.rs` is the entry point and calls each module’s run() function:
 ```
fn main() {
    loops::run();
    mutex::run();
    ownership::run();
    rwlock::run();
    serde::run();
    structures::run();
}
```
---
