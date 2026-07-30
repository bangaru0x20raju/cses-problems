# Rust References, Borrowing, and Moving

## References Don't Cascade Inward

When you have a reference to a nested data structure, the reference only protects the **outermost layer**. Inner layers are accessed by value (via dereferencing), and you must add `&` yourself to borrow them.

```rust
let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
let r: &Vec<Vec<i32>> = &v;

r           // type: &Vec<Vec<i32>>     ← reference
r[0]        // type: Vec<i32>           ← NOT a reference! [] dereferences
&r[0]       // type: &Vec<i32>          ← reference again (you add & yourself)
r[0][1]     // type: i32                ← NOT a reference
&r[0][1]    // type: &i32               ← reference (you add & yourself)
```

### Why `[]` Dereferences

The `[]` operator uses Rust's `Index` trait, which dereferences the result:

```rust
// graph[node] desugars to:
*graph.index(node)
//^--- this * dereferences it!

// So graph[node] gives you the VALUE, not a reference
```

---

## How `for` Loops Consume Things

The `for` loop has **two different behaviors**:

```rust
// WITHOUT &  →  calls .into_iter()  →  MOVES/CONSUMES the collection
for item in collection { }      // collection is GONE after this

// WITH &  →  calls .iter()  →  BORROWS the collection
for item in &collection { }    // collection still exists after this
```

### The Common Mistake with Nested References

```rust
let graph: &Vec<Vec<Edge>> = /* ... */;

// ❌ FAILS: graph[node] is Vec<Edge> (value) → for loop tries to MOVE it
for adj_node in graph[node] { }

// ✅ WORKS: &graph[node] is &Vec<Edge> (reference) → for loop BORROWS it
for adj_node in &graph[node] { }
// adj_node is &Edge (a reference to each Edge)
```

---

## `&` vs `&mut` vs Owned — What Each Allows

```
&T        →  "I'm borrowing this. I can LOOK at it."
&mut T    →  "I'm borrowing this. I can LOOK at and MODIFY it."
T (owned) →  "I OWN this. I can do anything, including DESTROY/MOVE it."
```

| Access | Read | Modify | Move out |
| :--- | :--- | :--- | :--- |
| `&T` | ✅ | ❌ | ❌ |
| `&mut T` | ✅ | ✅ | ❌ (must return it intact) |
| `T` (owned) | ✅ | ✅ | ✅ (but must keep the container valid) |

### Analogy: Borrowing a Car

```
&       →  You borrowed the car. You can drive it (read). Can't repaint it.
&mut    →  You borrowed the car. You can drive AND repaint it (modify).
            But you MUST return it! You can't sell it (move).
owned   →  Your friend GAVE you the car. It's yours. Sell it, scrap it, whatever.
```

### `&mut` Still Does NOT Allow Moving

```rust
fn process(graph: &mut Vec<Vec<Edge>>) {
    for adj_node in graph[0] { }  // ❌ STILL fails!
    // ERROR: cannot move out of `graph[0]` which is behind a mutable reference
}
```

Even with `&mut`, the compiler says: *"You're borrowing this graph. You can change it, but you can't rip out `graph[0]` and leave a hole."*

---

## Moving Out of a Vec

Even with **ownership**, you can't move a single element out via indexing because it would leave the Vec in a partially-invalid state (index 0 is gone but other indices exist).

```rust
fn process(graph: Vec<Vec<Edge>>) {
    for adj_node in graph[0] { }  // ❌ STILL fails!
    // Moving graph[0] out leaves a "hole" — Rust doesn't allow this
}
```

### Safe Ways to Move Out

```rust
// 1. Pop the last element
let last: Vec<Edge> = graph.pop().unwrap();         // ✅ Vec shrinks by 1

// 2. Remove at index (shifts everything after it)
let removed: Vec<Edge> = graph.remove(0);            // ✅ Vec shrinks by 1

// 3. Swap-remove (fast — swaps with last, then pops)
let removed: Vec<Edge> = graph.swap_remove(0);       // ✅ Vec shrinks by 1

// 4. Replace with something else (works via &mut too!)
let taken: Vec<Edge> = std::mem::replace(&mut graph[0], vec![]);
// ✅ Puts empty vec in its place — no hole

// 5. Consume the entire Vec
for inner_vec in graph {      // ✅ Moves ALL inner Vecs out, graph is gone
    for edge in inner_vec {   // ✅ Moves ALL edges out, inner_vec is gone
        // process edge
    }
}
```

---

## Quick Reference: Iterating Over Nested Structures

```rust
let graph: &Vec<Vec<Edge>> = /* ... */;

// Iterating outer Vec (graph is already &, so this borrows):
for row in graph { }           // ✅ row is &Vec<Edge>

// Iterating inner Vec:
for item in graph[i] { }      // ❌ tries to move
for item in &graph[i] { }     // ✅ item is &Edge

// If graph is owned (not a reference):
let graph: Vec<Vec<Edge>> = /* ... */;
for item in &graph[i] { }     // ✅ borrows, item is &Edge
for item in graph[i] { }      // ❌ still fails (can't leave a hole in the Vec)
```

## TL;DR

- `&` on the outer container does **not** cascade to inner elements — you must add `&` yourself
- `[]` (Index operator) **dereferences** — `graph[i]` gives the value, not a reference
- `for x in collection` **moves** — use `for x in &collection` to borrow instead
- `&mut` allows modification but **not** moving — you must return borrowed data intact
- To move out of a Vec, use `.pop()`, `.remove()`, `.swap_remove()`, or `std::mem::replace()`
