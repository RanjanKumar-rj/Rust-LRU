
# LRU Cache in Rust (Work in Progress)

## 📌 What is LRU?
LRU stands for **Least Recently Used**. It is a cache eviction policy that keeps only a limited number of items in memory.  
When the cache reaches its maximum capacity and a new item needs to be added:
- The **least recently used** item is removed from the cache.
- The **most recently used** items are retained.

This ensures that frequently accessed items remain available while old, unused items get evicted.

---

## 🛠️ How LRU Works
An LRU cache typically supports two main operations:
1. **Get(key) → value**
   - Returns the value associated with the key if present.
   - Marks the key as "recently used".

2. **Put(key, value)**
   - Inserts or updates the key-value pair.
   - Marks the key as "recently used".
   - If the cache exceeds its capacity, the **least recently used** key is evicted.

Both operations should ideally run in **O(1) time**.

---

## 📚 Data Structures Behind LRU
To achieve O(1) operations, LRU caches usually combine:
- **HashMap (K → Node reference)**  
  Provides fast lookups to check if a key exists.

- **Doubly Linked List**  
  Maintains the usage order:
  - **Head** → Most recently used (MRU).  
  - **Tail** → Least recently used (LRU).  

When a key is accessed:
- Its node is moved to the **head**.  
- When the cache is full, the **tail node** is evicted.

---

## 🧩 Current Progress
This repository is being built step by step:
- ✅ Implementing a **Doubly Linked List** in Rust using `Rc<RefCell<T>>`.  
- 🔜 Integrating with a **HashMap** to form a complete **LRU Cache**.  

The doubly linked list is the backbone that allows efficient reordering of nodes when cache entries are accessed.

---

## 📈 Roadmap
- [x] Implement a doubly linked list.  
- [ ] Add deletion methods (`delete_from_front`, `delete_from_end`, `delete_node`).  
- [ ] Combine with `HashMap` for O(1) lookups.  
- [ ] Build full LRU cache with `get` and `put`.  
- [ ] Add unit tests and benchmarks.  

---

