# Generic Inventory System in Rust

## Description

This is a Rust program that manages items in an inventory using HashMap.  
The inventory is generic, so it can store any type of item, but each item must
implement a DisplayItem trait.

---

## Features

- Generic Inventory <T>
- Uses HashMap<String, T> to store items
- Custom DisplayItem trait
- Error handling using Result and InventoryError
- Prevents duplicate IDs
- Displays all items

---

## Concepts Used

- Generics
- Traits
- Trait Bounds (T: DisplayItem + Clone)
- HashMap
- Result and custom enum for errors

---

## Program Structure

- DisplayItem → Trait for display method  
- Product → Example item struct  
- InventoryError → Custom error enum  
- Inventory<T> → Generic inventory struct  
- add_item() → Adds item with duplicate check  
- display_all() → Shows all stored items  

---


## Short Explanation

Inventory is a generic struct that stores items in HashMap using ID as key.  
Each item must implement DisplayItem trait.  
add_item checks empty and duplicate IDs using Result.  
display_all loops through HashMap and prints all items.

HashMap is used because it gives fast access using ID.

---
