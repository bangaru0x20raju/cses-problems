# Rust Auto-Referencing: How Methods Get `&self` Without You Passing a Reference

When you call a method using dot (`.`) syntax, Rust **automatically adds `&`, `&mut`, or `*`** to the receiver (the thing before the dot) to match the method signature. This is called **auto-referencing** (or "autoref").

---

## The Question

If a function expects a reference, we must pass a reference. So how does a method taking `&self` work without us calling it with a reference?

```rust
struct Point { x: i32, y: i32 }

impl Point {
    fn distance(&self) -> f64 {  // takes &self (a reference)
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

fn main() {
    let p = Point { x: 3, y: 4 };

    // You write this:
    p.distance();        // ✅ Works! Rust auto-converts to (&p).distance()

    // What Rust actually does behind the scenes:
    (&p).distance();     // ✅ This is what the compiler generates

    // You can also call it as a regular function (no auto-ref magic):
    Point::distance(&p); // ✅ Here YOU must pass the reference explicitly
}
```

---

## Auto-Ref Only Works for the Receiver (Before the Dot)

Rust's auto-referencing **only** applies to the **receiver** (the value before `.`). It does **not** apply to regular function arguments:

```rust
fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

fn main() {
    let p = Point { x: 1, y: 2 };

    // Method call — auto-ref kicks in for the receiver:
    p.distance();          // ✅ Rust adds & automatically → (&p).distance()

    // Regular function call — NO auto-ref:
    print_point(p);        // ❌ ERROR: expected &Point, found Point
    print_point(&p);       // ✅ You must add & yourself
}
```

---

## What Does the Compiler Try?

When you write `p.method()`, the compiler tries these in order until one matches:

| Step | What it tries | Matches if method takes |
| :--- | :--- | :--- |
| 1 | `p.method()` | `self` (takes ownership) |
| 2 | `(&p).method()` | `&self` (shared reference) |
| 3 | `(&mut p).method()` | `&mut self` (mutable reference) |
| 4 | `(*p).method()` | dereferences first (e.g., if `p` is a `Box<T>`) |

---

## Concrete Example with `eq`

```rust
// When you write:
point.eq(&other)

// The method signature is:  fn eq(&self, other: &Self) -> bool
//                               ^^^^^
//                               &self requires a reference

// Rust auto-converts it to:
Point::eq(&point, &other)
//        ^^^^^^
//        Rust adds this & for you, but ONLY for the receiver (self)
//                 ^^^^^^
//                 For other args, YOU must add & yourself
```

---

## TL;DR

- The dot (`.`) operator is special — it automatically borrows/dereferences the **receiver** to match the method signature.
- Regular function arguments **don't** get this treatment — you must pass `&` yourself.
- This is why `p.distance()` works even though `distance` takes `&self`.
