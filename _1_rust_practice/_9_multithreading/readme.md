# Multithreading in Rust

## About This Project

In this program, I have implemented multithreading using Rust.
Here, 6 threads are running at the same time and working on shared data.

I used `Arc` and `RwLock` so that multiple threads can safely access and modify the same vector.

The main objective of this program is to understand how concurrency and shared memory work in Rust.

---

## Structure Used

```rust
struct MultiThread {
    id: i32,
    record_added_time: SystemTime,
    thread_id: String,
}
```

### Field Description

* **id** → Unique id generated using a global counter
* **record_added_time** → Time when the record was added
* **thread_id** → Identifier of the thread

---

## Threads and Their Work

### Thread 1 – Record Creator

* Adds a new record every 10 seconds.
* Generates unique id using a counter.

### Thread 2 – State Printer

* Continuously prints all the records present in memory.

### Thread 3 – Even Record Cleaner

* Removes records:

  * With even id
  * Older than 20 seconds

### Thread 4 – Odd Record Cleaner

* Removes records:

  * With odd id
  * Older than 20 seconds

### Thread 5 – Even Counter

* Counts and prints total number of even id records.

### Thread 6 – Odd Counter

* Counts and prints total number of odd id records.

---

## Concepts Used

* `Arc` – Used to share ownership of data between threads.
* `RwLock` – Allows multiple readers or one writer at a time.
* `thread::spawn()` – Used to create new threads.
* `join()` – Makes main thread wait for other threads.

---

## Output

* Records are added every 10 seconds.
* Records are printed continuously.
* Even and odd record counts are displayed.
* Records older than 20 seconds are removed automatically.

---

