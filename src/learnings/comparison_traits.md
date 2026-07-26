# Rust Comparison Traits: PartialEq, Eq, PartialOrd, and Ord

Rust has 4 traits for comparing values. They form two pairs:

| | Equality (`==`, `!=`) | Ordering (`<`, `>`, `<=`, `>=`) |
| :--- | :--- | :--- |
| **Partial** (some values may not be comparable) | `PartialEq` | `PartialOrd` |
| **Total** (all values are always comparable) | `Eq` | `Ord` |

---

## 1. PartialEq — "Can I check if two values are equal?"

Enables `==` and `!=` operators. Most types implement this.

**The catch:** A type with `PartialEq` is allowed to have values that are **not equal to themselves**.

### Example: NaN in Floats
```rust
fn main() {
    let x = f64::NAN;
    println!("{}", x == x); // prints: false  ← NaN is not equal to itself!
}
```
Floats (`f32`, `f64`) implement `PartialEq` but **not** `Eq`, because `NaN == NaN` is `false`.

### Analogy: Blank ID Cards
Imagine ID cards where some cards are blank (unissued):
```rust
struct IDCard {
    number: Option<u32>, // None = blank card
}
```
- `IDCard { number: Some(123) } == IDCard { number: Some(123) }` → **true** ✅
- `IDCard { number: None } == IDCard { number: None }` → **false** ❌ (a blank card has no identity, it's not even equal to itself)

This is `PartialEq` — equality works for *most* values, but breaks down for some special values.

---

## 2. Eq — "Every value is guaranteed to equal itself"

`Eq` is a **marker trait** (no extra methods). It simply promises: **for every value `x` of this type, `x == x` is always `true`**.

Types like `i32`, `String`, `bool`, `char` all implement `Eq` because:
- `5 == 5` → always true
- `"hello" == "hello"` → always true

### When is `Eq` required? (Compiler will reject `PartialEq` alone)

| Data Structure | Why it needs `Eq` |
| :--- | :--- |
| **`HashMap` keys** | If a key isn't equal to itself, you can never look it up again after inserting it |
| **`HashSet` elements** | Same reason — the set can never find the element to check if it already exists |

### Example: Compiler Error without `Eq`
```rust
use std::collections::HashSet;

#[derive(PartialEq, Hash)]  // PartialEq but NOT Eq
struct User { id: u32 }

fn main() {
    let mut users = HashSet::new();
    users.insert(User { id: 1 }); // ❌ ERROR: the trait `Eq` is not implemented for `User`
}
```
**Fix:** Add `Eq` → `#[derive(PartialEq, Eq, Hash)]`

---

## 3. PartialOrd — "Can I compare which value is bigger?"

Enables `<`, `>`, `<=`, `>=` operators. Returns `Option<Ordering>` — the `None` means "these two values cannot be compared".

### Analogy: Apples and Oranges
Imagine a `Fruit` type that can be an Apple or an Orange:
- `Apple(150g) > Apple(120g)` → **Some(Greater)** ✅ (apples can be compared by weight)
- `Orange(200g) > Orange(180g)` → **Some(Greater)** ✅ (oranges can be compared by weight)
- `Apple(150g) > Orange(200g)` → **None** ❌ (you can't compare apples and oranges!)

This is `PartialOrd` — some pairs of values simply cannot be ordered.

### Example: NaN in Floats
```rust
fn main() {
    let result = f64::NAN.partial_cmp(&5.0);
    println!("{:?}", result); // prints: None  ← NaN can't be compared to anything
}
```

### When is `PartialOrd` enough?
- Simple comparisons in `if` statements: `if score > threshold { ... }`
- Checking ranges / boundaries: `if value >= min && value <= max { ... }`
- Sorting with a custom comparator: `vec.sort_by(|a, b| a.partial_cmp(b).unwrap())`

---

## 4. Ord — "Any two values can always be ordered"

Returns a strict `Ordering` (`Less`, `Greater`, or `Equal`) — **never** `None`. Every pair of values has a definitive order.

### When is `Ord` required? (Compiler will reject `PartialOrd` alone)

| Data Structure / Operation | Why it needs `Ord` |
| :--- | :--- |
| **`vec.sort()`** | Sorting must place every element in a definitive position |
| **`BTreeMap` keys** | Binary search trees need to decide left or right for every node |
| **`BTreeSet` elements** | Same reason as `BTreeMap` |
| **`BinaryHeap`** | Priority queue must always know which element is the largest |

### Example: Compiler Error without `Ord`
```rust
fn main() {
    let mut floats = vec![3.0_f64, 1.0, 2.0];
    floats.sort(); // ❌ ERROR: the trait `Ord` is not implemented for `f64`
}
```
**Fix:** Use a custom comparator instead:
```rust
floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
// or the cleaner way:
floats.sort_by(|a, b| a.total_cmp(b));
```

---

## 5. Quick Reference

```
PartialEq          Eq
  (==, !=)           (== is always reflexive: x == x)
  ↑                  ↑
  │                  │ extends PartialEq
  │                  │
  Used for:          Used for:
  - if a == b        - HashMap keys
  - match guards     - HashSet elements


PartialOrd          Ord
  (<, >, <=, >=)     (total ordering, never returns None)
  ↑                  ↑
  │                  │ extends PartialOrd
  │                  │
  Used for:          Used for:
  - if a > b         - vec.sort()
  - range checks     - BTreeMap / BTreeSet
  - sort_by()        - BinaryHeap
```

### One-liner Summary
- **`PartialEq`** → "I can check equality, but some weird values might not equal themselves" (e.g., `NaN`)
- **`Eq`** → "Every value always equals itself" (required for `HashMap` / `HashSet`)
- **`PartialOrd`** → "I can compare most values, but some pairs are incomparable" (e.g., `NaN` vs any number)
- **`Ord`** → "Any two values can always be ordered" (required for sorting / `BTreeMap` / `BinaryHeap`)

---

## 6. Methods to Implement

### `PartialEq` — 1 required method
```rust
trait PartialEq {
    fn eq(&self, other: &Self) -> bool;       // ✅ REQUIRED — defines ==

    fn ne(&self, other: &Self) -> bool { .. }  // 🔧 Optional — default is !self.eq(other)
}
```
**Manual implementation example:**
```rust
struct Point { x: i32, y: i32 }

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    // ne() is auto-generated, no need to write it
}
```

### `Eq` — 0 methods (marker trait)
```rust
trait Eq: PartialEq {
    // Nothing to implement!
    // It's a promise: "I guarantee x == x is always true for my type"
}
```
**Manual implementation example:**
```rust
impl Eq for Point {}  // That's it! Just an empty impl block.
```

### `PartialOrd` — 1 required method
```rust
trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;  // ✅ REQUIRED

    fn lt(&self, other: &Self) -> bool { .. }   // 🔧 Optional — default uses partial_cmp
    fn le(&self, other: &Self) -> bool { .. }   // 🔧 Optional
    fn gt(&self, other: &Self) -> bool { .. }   // 🔧 Optional
    fn ge(&self, other: &Self) -> bool { .. }   // 🔧 Optional
}
```
**Manual implementation example:**
```rust
use std::cmp::Ordering;

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare by x first, then by y
        Some(self.cmp(other))  // delegate to Ord if you have it
    }
    // lt(), le(), gt(), ge() are auto-generated from partial_cmp
}
```

### `Ord` — 1 required method
```rust
trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;  // ✅ REQUIRED — must return Less, Greater, or Equal

    fn max(self, other: Self) -> Self { .. }    // 🔧 Optional
    fn min(self, other: Self) -> Self { .. }    // 🔧 Optional
    fn clamp(self, min: Self, max: Self) -> Self { .. }  // 🔧 Optional
}
```
**Manual implementation example:**
```rust
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by x first, then by y (lexicographic order)
        self.x.cmp(&other.x)
            .then(self.y.cmp(&other.y))
    }
}
```

### Quick Summary Table

| Trait | Required Method | Return Type | Depends On |
| :--- | :--- | :--- | :--- |
| **`PartialEq`** | `fn eq(&self, other: &Self)` | `bool` | — |
| **`Eq`** | *(none — marker trait)* | — | `PartialEq` |
| **`PartialOrd`** | `fn partial_cmp(&self, other: &Self)` | `Option<Ordering>` | `PartialEq` |
| **`Ord`** | `fn cmp(&self, other: &Self)` | `Ordering` | `Eq` + `PartialOrd` |

### Shortcut: Use `#[derive(...)]`
In most cases you don't need to write these manually. Just derive them:
```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point { x: i32, y: i32 }
```
This auto-generates all four traits by comparing fields in declaration order (first `x`, then `y`).

