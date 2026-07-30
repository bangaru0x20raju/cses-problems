# Rust Pattern Matching and Bindings

Patterns are used to **test the shape** of a value and **extract parts** of it. The same pattern matching rules work **everywhere** in Rust.

---

## Where Patterns Appear

Patterns are used in 6 places, and they all work the same way:

```rust
// 1. match arms
match value {
    PATTERN => expression,
}

// 2. let bindings
let PATTERN = expression;

// 3. if let
if let PATTERN = expression { }

// 4. while let
while let PATTERN = expression { }

// 5. for loops
for PATTERN in iterator { }

// 6. function parameters
fn foo(PATTERN: Type) { }
```

---

## Pattern Types

### 1. Literal Patterns

```rust
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}
```

### 2. Variable Binding (Captures the Value)

```rust
match x {
    n => println!("got: {n}"),  // n captures the value
}

let name = "Raju";  // name captures "Raju"
```

### 3. Wildcard `_` (Ignore the Value)

```rust
match x {
    _ => println!("don't care"),
}

let (_, b) = (1, 2);  // ignore first element
```

### 4. Multiple Values with `|` (OR)

```rust
match x {
    1 | 2 | 3 => println!("one, two, or three"),
    _ => println!("other"),
}
```

### 5. Range Patterns

```rust
match x {
    1..=5 => println!("one through five"),
    _ => println!("other"),
}
```

### 6. `@` Binding (Test + Capture)

```rust
match x {
    n @ 1..=5 => println!("{n} is between 1 and 5"),
    n => println!("{n} is out of range"),
}
```

---

## Destructuring Patterns

### Tuples

```rust
let point = (3, -5);

// let binding
let (x, y) = point;

// match
match point {
    (0, 0) => println!("origin"),
    (x, 0) => println!("on x-axis at {x}"),
    (0, y) => println!("on y-axis at {y}"),
    (x, y) => println!("at ({x}, {y})"),
}

// for loop
for (index, value) in vec.iter().enumerate() { }
```

### Structs

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 3, y: 7 };

// let binding
let Point { x, y } = p;

// match
match p {
    Point { x: 0, y } => println!("on y-axis at {y}"),
    Point { x, y: 0 } => println!("on x-axis at {x}"),
    Point { x, y } => println!("at ({x}, {y})"),
}
```

### Enums

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

match shape {
    Shape::Circle(r) => println!("radius {r}"),
    Shape::Rectangle(w, h) => println!("{w}x{h}"),
}
```

### Option and Result

```rust
// Option<T> is just an enum: Some(value) or None
if let Some(v) = maybe_value {
    println!("got {v}");
}

// Result<T, E> is just an enum: Ok(value) or Err(error)
match result {
    Ok(v) => println!("success: {v}"),
    Err(e) => println!("error: {e}"),
}
```

---

## Reference Destructuring (Peeling Off `&`)

This is where pattern matching becomes very practical in everyday Rust code.

When you iterate over `&Vec<usize>`, each element is `&usize`. You can use a pattern to **peel off the `&`**:

```rust
// WITHOUT destructuring — adj_node is &usize
for adj_node in &graph[node] {
    in_degree[*adj_node] -= 1;     // need * to dereference
}

// WITH destructuring — &adj_node peels off the &, so adj_node is usize
for &adj_node in &graph[node] {
    in_degree[adj_node] -= 1;      // no * needed!
}
```

### How It Works

```rust
let reference: &usize = &42;

let adj_node = reference;      // adj_node is &usize (the whole reference)
let &adj_node = reference;     // adj_node is usize (destructured, & peeled off)
//  ^
//  "peel off the & and give me what's inside"
```

### This Works the Same Everywhere

```rust
// let binding
let &x = &42;                          // x is usize

// match
match &42 {
    &value => println!("{value}"),      // value is usize
}

// if let
if let &value = &42 {
    println!("{value}");                // value is usize
}

// for loop
for &value in &vec![1, 2, 3] {
    println!("{value}");                // value is usize
}

// function parameter
fn foo(&x: &i32) {
    println!("{x}");                    // x is i32, not &i32
}
```

### ⚠️ Only Works for Copy Types!

Destructuring `&` with a pattern **copies** the value out. This only works for types that implement `Copy` (like `usize`, `i32`, `bool`, `char`, `f64`).

For non-Copy types like `String` or `Vec`, you **cannot** peel off the `&`:

```rust
let names = vec!["Raju".to_string(), "Alice".to_string()];

// ✅ Works — name is &String (borrowed)
for name in &names { }

// ❌ ERROR — can't copy/move String out of a reference
for &name in &names { }
```

---

## Match Guards (Extra Conditions)

Add `if` conditions to a pattern:

```rust
match x {
    n if n < 0 => println!("negative"),
    n if n % 2 == 0 => println!("{n} is even"),
    n => println!("{n} is odd"),
}
```

---

## Returning Values from Match

`match` is an **expression** — it produces a value:

```rust
// Assign to variable
let label = match x {
    1 => "one",
    2 => "two",
    _ => "other",
};

// Return from function
fn describe(x: i32) -> &'static str {
    match x {
        1 => "one",
        _ => "other",
    }
}

// Multi-line arms — last expression is the return value
let result = match x {
    1 => {
        println!("processing...");
        "one"    // ← return value (no semicolon!)
    },
    _ => "other",
};
```

### Break/Return from Outer Scope

```rust
// Return from the entire function inside a match arm
fn find(data: &[i32], target: i32) -> bool {
    for &val in data {
        match val == target {
            true => return true,   // exits the FUNCTION
            false => continue,      // continues the FOR loop
        }
    }
    false
}

// Break from a loop with a value
let result = loop {
    match get_input() {
        Some(v) if v > 0 => break v,   // break WITH a value
        Some(_) => continue,
        None => break -1,
    }
};
```

---

## Shorthand: `if let` and `while let`

When you only care about **one pattern**:

```rust
// Instead of:
match option {
    Some(v) => println!("{v}"),
    None => {},
}

// Use if let:
if let Some(v) = option {
    println!("{v}");
}

// Loop until pattern fails:
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

---

## Exhaustiveness

`match` must cover **every possible value**:

```rust
match x {
    1 => println!("one"),
}
// ❌ ERROR: non-exhaustive patterns

match x {
    1 => println!("one"),
    _ => println!("other"),   // ✅ catches everything else
}
```

Enums don't need `_` if all variants are listed:
```rust
match direction {
    Direction::Up => ...,
    Direction::Down => ...,
    Direction::Left => ...,
    Direction::Right => ...,
    // ✅ All variants covered, no _ needed
}
```

---

## Quick Reference

```
PATTERN                          WHAT IT MATCHES
───────                          ───────────────
42                               exact value 42
_                                anything (ignore)
x                                anything (bind to x)
1 | 2 | 3                       1 or 2 or 3
1..=5                            range 1 to 5 inclusive
n @ 1..=5                        range + bind to n
(a, b)                           tuple, bind parts
(_, b)                           tuple, ignore first
Point { x, y }                  struct, bind fields
Some(v)                          Option with value
None                             Option without value
Ok(v)                            Result success
Err(e)                           Result error
&x                               reference, peel off & (Copy types only)
n if n > 0                       value + guard condition
```
