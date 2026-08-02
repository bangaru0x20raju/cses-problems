# Expressions, Statements, and `if/else` in Rust

## 1. Expressions vs. Statements

In Rust, the distinction between **expressions** and **statements** is fundamental:

- **Expression:** A piece of code that **evaluates to a value**.
- **Statement:** An instruction that performs an action but **does NOT evaluate to a value** (or evaluates to the unit type `()`).

### Quick Comparison Table

| Feature | Expression | Statement |
| :--- | :--- | :--- |
| **Returns a value?** | **YES** (e.g., `5`, `"hello"`, `x + y`) | **NO** (evaluates to `()`) |
| **Ends with `;`?** | **NO** | **YES** (usually) |
| **Can be assigned to variable?** | **YES** (`let x = <expression>;`) | **NO** |

---

## 2. Rust is an "Expression-Oriented Language"

In languages like C, C++, or Java, constructs like `if/else` or `{}` blocks are strictly statements that control code flow.

In Rust, **`if/else`**, **`match`**, and **blocks `{}` are expressions** that produce values.

### Tail Expressions (Implicit Returns)
Functions and blocks evaluate to their **tail expression**—the last line inside a block without a trailing semicolon.

```rust
fn max(a: i32, b: i32) -> i32 {
    // The entire `if/else` block is the tail expression of the function.
    if a > b {
        a // Tail expression of `if` branch (evaluates to i32)
    } else {
        b // Tail expression of `else` branch (evaluates to i32)
    }
}
```

---

## 3. The Semicolon `;` Rule

In Rust, **adding a semicolon `;` converts an expression into a statement**, discarding its evaluated value (turning it into `()`).

### Example 1: Function Return Success vs Failure

```rust
// ✅ COMPILES: Tail expression `10` evaluates to i32
fn get_number() -> i32 {
    10 
}

// ❌ FAILS TO COMPILE: Semicolon converts `10` into a statement evaluating to `()`
fn get_number_broken() -> i32 {
    10; 
}
```

### Example 2: `if/else` Semicolon Trap

```rust
// ❌ FAILS: Semicolon at the end of `if/else` converts the result into `()`
fn get_val(cond: bool) -> i32 {
    if cond {
        10
    } else {
        20
    }; // <-- Semicolon causes type error (expected i32, found ())
}
```

---

## 4. What is the Unit Type `()`?

When an `if` condition or block does not return a value, it evaluates to **`()`**, known as the **Unit Type**.

- `()` represents an **empty tuple**.
- It is Rust's version of `void` (indicating "no meaningful value").

### 3 Cases Where `if` Evaluates to `()`

1. **`if` without an `else` branch:**
   ```rust
   let x = 5;
   let res: () = if x > 0 {
       println!("Positive");
   }; // If x <= 0, no code runs, so Rust defaults the evaluation to ()
   ```
2. **All statements end with `;` inside branches:**
   ```rust
   let res: () = if cond {
       println!("Hello"); // ends with ; -> returns ()
   } else {
       println!("World"); // ends with ; -> returns ()
   };
   ```
3. **Empty `{}` blocks:**
   ```rust
   let res: () = if cond {} else {}; // Evaluates to ()
   ```

---

## 5. Type Matching Rule Across `if / else` Arms

Because `if/else` is an expression, **both arms MUST evaluate to the exact same type**.

```rust
// ❌ COMPILER ERROR: Mismatched arm types (i32 vs ())
let x = if condition {
    10                 // Type: i32
} else {
    println!("false"); // Type: () (due to ;)
};

// ✅ FIX: Ensure both arms return the same type
let x = if condition {
    10
} else {
    20
};
```

---

## 6. Other Expression Constructs in Rust

### A. `match` Expressions
```rust
let name = match number {
    1 => "one",
    2 => "two",
    _ => "other",
};
```

### B. Block `{}` Expressions
```rust
let area = {
    let width = 5;
    let height = 10;
    width * height // Block evaluates to 50
};
```

### C. `loop` with `break value`
```rust
let first_even = loop {
    if num % 2 == 0 {
        break num; // Yields `num` out of loop expression
    }
};
```
