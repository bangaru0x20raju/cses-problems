# Rust Structs, Methods, Associated Functions, and `self` Receivers

## 1. Rust Structs Overview

A `struct` (structure) is a custom data type that packages together related values. Rust provides 3 types of structs:

### Named-Field Struct
The most common type. Fields are explicitly named.
```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

### Tuple Struct
Fields are unnamed and accessed via positional index (`.0`, `.1`, etc.).
```rust
struct Color(i32, i32, i32);
let red = Color(255, 0, 0);
println!("Red component: {}", red.0);
```

### Unit-Like Struct
Contains no fields. Useful when implementing traits without storing state.
```rust
struct AlwaysEqual;
```

---

## 2. Associated Functions vs. Methods

Inside an `impl` (implementation) block:

### Associated Functions
Functions defined in an `impl` block that **do not take any form of `self`** as their first parameter. They do not operate on an instance directly.
- **Calling syntax:** `Type::function_name(...)`
- **Common use case:** Constructors (`new`, `with_capacity`, `open`, etc.)

```rust
impl Rectangle {
    // Constructor (Associated Function)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// Called using namespace syntax:
let rect = Rectangle::new(30, 50);
```

### Methods
Associated functions whose **first parameter is a variant of `self`** (`&self`, `&mut self`, `self`, or `mut self`).
- **Calling syntax:** `instance.method_name(...)` (dot notation)

```rust
impl Rectangle {
    // Method (operates on an instance of Rectangle)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Called using dot notation:
let a = rect.area();
```

---

## 3. `Self` vs. `self` Keywords & Receivers

### `Self` (Capital 'S') — The Type Alias
`Self` is a type alias for the type implementing the `impl` block.

```rust
impl Rectangle {
    // Returning `Self` is equivalent to returning `Rectangle`
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
```

---

### Receiver Parameters (`self` variants)

| Parameter | Full Type Signature | Borrow/Ownership Mode | Access Level | Typical Use Cases |
| :--- | :--- | :--- | :--- | :--- |
| **`&self`** | `self: &Self` | Shared/Immutable reference | Read-only access to fields | Reading state, calculating values, getters |
| **`&mut self`** | `self: &mut Self` | Exclusive/Mutable reference | Read & write access to fields | Modifying internal fields or state in-place |
| **`self`** | `self: Self` | Takes full ownership (Moves) | Consumes instance | Builder pattern step, type conversions (`into_*`), cleanup/closing resources |
| **`mut self`** | `mut self: Self` | Takes full ownership + local mutation | Modifies before consuming | Transforming owned data before dropping/returning |

---

### Key Receiver Examples

```rust
struct User {
    username: String,
    active: bool,
}

impl User {
    // 1. &self - Read-only access (Borrowing)
    fn get_name(&self) -> &str {
        &self.username
    }

    // 2. &mut self - In-place mutation (Exclusive Borrowing)
    fn deactivate(&mut self) {
        self.active = false;
    }

    // 3. self - Takes ownership (Consumes instance)
    fn consume(self) -> String {
        println!("Consuming user {}", self.username);
        self.username // User instance is dropped at the end of this method
    }
}
```

---

## 4. Complete Code Example

```rust
#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Associated Function: Constructor returning `Self`
    fn open(owner: &str, initial_deposit: f64) -> Self {
        Self {
            owner: owner.to_string(),
            balance: initial_deposit,
        }
    }

    // Method with &self: Immutable borrow for checking state
    fn balance(&self) -> f64 {
        self.balance
    }

    // Method with &mut self: Mutable borrow to alter state
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Method with self: Consumes the account instance completely
    fn close(self) -> f64 {
        println!("Closing account for {}", self.owner);
        self.balance // Account is dropped here
    }
}

fn main() {
    // 1. Create instance using Associated Function
    let mut account = BankAccount::open("Alice", 100.0);

    // 2. Call Method taking &self
    println!("Balance: {}", account.balance());

    // 3. Call Method taking &mut self
    account.deposit(50.0);
    println!("New Balance: {}", account.balance());

    // 4. Call Method taking self (Consumes account)
    let payout = account.close();
    println!("Final payout: {}", payout);

    // Error: account cannot be used anymore because it was moved into `close(self)`
    // println!("{:?}", account);
}
```
