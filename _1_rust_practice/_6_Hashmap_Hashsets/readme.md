# Rust HashMap and HashSet Program

## Aim

To create a custom struct in Rust using HashMap and HashSet and perform operations like:

- try_reserve  
- insert  
- clone  
- take  
- retain  
- extend  

---

## Description

In this program, a custom struct called `Student` is created.

The struct contains:

- `id_name` → HashMap (stores student id and name)
- `groups` → HashSet (stores group names)

The program demonstrates how to:

1. Reserve memory using `try_reserve`
2. Insert values into HashMap and HashSet
3. Clone the struct
4. Use `std::mem::take()` to move HashMap data
5. Use `retain()` to keep selected values
6. Use `extend()` to add new values
7. Print the final output

---

## Custom Struct

```rust
struct Student {
    id_name: HashMap<u32, String>,
    groups: HashSet<String>,
}
