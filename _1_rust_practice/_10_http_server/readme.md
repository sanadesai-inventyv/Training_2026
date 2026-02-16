
# Rust API (Axum + RwLock + JSON Storage)

This project is a simple REST API built using:

- Rust
- Axum (0.7)
- Tokio
- RwLock (for multithreading)
- JSON file storage
- UUID

It performs basic CRUD operations:
- GET
- POST
- PUT
- DELETE

---

## 📁 Project Structure

```

src/
│── main.rs
│── model.rs
│── api.rs
│── handle.rs
│── routes.rs
students.json

````

---

## How to Run
 Run:

```bash
cargo run
````

Server will start at:

```
http://127.0.0.1:4500
```

---

##  API Endpoints

### 1. Get All Students

```bash
curl -X GET http://127.0.0.1:4500/students
```

---

###  2. Create Student

```bash
curl -X POST http://127.0.0.1:4500/students \
-H "Content-Type: application/json" \
-d '{ "name": "Sana", "email": "sana@test.com" }'
```

Response: `201 Created`

---

### 3. Update Student

```bash
curl -X PUT http://127.0.0.1:4500/students/{id} \
-H "Content-Type: application/json" \
-d '{ "name": "Updated Name", "email": "updated@test.com" }'
```

Response: `200 OK`

---

###  4. Delete Student

```bash
curl -X DELETE http://127.0.0.1:4500/students/{id}
```

Response: `200 OK`

---

## How It Works

* Students are stored in `students.json`
* Data is loaded at server startup
* `Arc<RwLock<Vec<Student>>>` is used for shared state
* Multiple GET requests can run simultaneously
* POST/PUT/DELETE use write lock
* UUID is generated for each new student

---

## Multithreading

This project uses:

```
Arc + RwLock + Tokio Runtime
```

* `read()` for GET
* `write()` for POST/PUT/DELETE

Ensures thread-safe concurrent access.

---

## Dependencies

* axum
* tokio
* serde
* serde_json
* uuid


---

