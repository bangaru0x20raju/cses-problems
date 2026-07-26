# Rust I/O Performance: BufReader and BufWriter

## The Problem with `io::stdin().read_line()`

Every call to `stdin().read_line()` makes a **system call** to the OS. System calls are expensive because they switch from your program to the OS kernel and back.

If you have 150,000 edges and each needs a `read_line()`, that's **150,000+ system calls**.

```
Your Program          OS Kernel
    │                    │
    ├── read_line() ────→│  (system call 1)
    │←── "1 2 5\n" ──────│
    ├── read_line() ────→│  (system call 2)
    │←── "2 3 8\n" ──────│
    ...
    150,000 system calls! SLOW!
```

---

## How `BufReader` Fixes This

`BufReader` wraps stdin with an **internal buffer** (default 8KB). Instead of reading one line from the OS each time, it reads a **large chunk** into memory in one system call, then serves your `read_line()` calls from that in-memory buffer instantly.

```
Your Program          BufReader (in memory)       OS Kernel
    │                    │                           │
    │                    ├── read 8KB ──────────────→│  (system call 1)
    │                    │←── 8KB of data ───────────│
    ├── read_line() ────→│  (from buffer, instant!)  │
    │←── "1 2 5\n" ──────│                           │
    ├── read_line() ────→│  (from buffer, instant!)  │
    │←── "2 3 8\n" ──────│                           │
    ...
    Only ~20 system calls for 150,000 lines!
```

### The 8KB Buffer Auto-Refills — You Never Manage It

The 8KB is NOT a limit on your input. When the buffer runs out, `BufReader` **automatically refills** it from the OS. You never need to think about it.

Think of it like a **glass connected to a tank**:
- You drink from the glass (read lines from buffer)
- When the glass is empty, it **auto-refills** from the tank (OS reads more data)
- You just keep drinking — you never manually refill the glass
- You only stop when the **tank is empty** (EOF)

```
Input from OS (say 20KB total):

Step 1:   BufReader reads first 8KB from OS (system call 1)
          You read lines from this buffer...

Step 101: Buffer empty → BufReader reads next 8KB (system call 2) ← AUTOMATIC!
          You keep reading lines...

Step 200: Buffer empty → BufReader reads last 4KB (system call 3) ← AUTOMATIC!
          You keep reading lines...

Final:    No more data → lines().next() returns None (EOF)
```

---

## How to Use BufReader

```rust
use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let n_and_m: Vec<usize> = first_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..n_and_m[1] {
        let line = lines.next().unwrap().unwrap();
        // parse edge...
    }
}
```

---

## Alternative: Read Everything at Once

The fastest approach — read all input in a single system call, then parse tokens:

```rust
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap(); // 1 system call!

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..m {
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: u64 = iter.next().unwrap().parse().unwrap();
        // build graph...
    }
}
```

---

## BufWriter: Same Idea for Output

Each `println!` flushes to stdout immediately (system call per print). Use `BufWriter` to batch writes:

```rust
use std::io::{self, Write, BufWriter};

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for i in 1..=100000 {
        write!(out, "{} ", i).unwrap(); // Buffered, not flushed every time
    }
    writeln!(out).unwrap();
    // BufWriter flushes automatically when dropped
}
```

---

## EOF Detection

If you don't know how many lines to read:

```rust
// Method 1: read_line() returns Ok(0) bytes at EOF
let mut line = String::new();
let bytes_read = reader.read_line(&mut line).unwrap();
if bytes_read == 0 {
    break; // No more input!
}

// Method 2: lines() iterator returns None at EOF
for line in reader.lines() {
    let line = line.unwrap();
    // process...
}
// Loop ends automatically when input is exhausted

// Method 3: split_whitespace iterator returns None
while let Some(token) = iter.next() {
    // process token...
}
```

---

## Speed Comparison

| Method | System Calls (150K lines) | Speed |
| :--- | :--- | :--- |
| `stdin().read_line()` per line | ~150,000 | Slow |
| `BufReader` wrapping stdin | ~20 | Fast |
| `read_to_string()` (all at once) | 1 | Fastest |

## TL;DR

- **`read_line()`** = 1 system call per line = slow for large input
- **`BufReader`** = reads big chunks into memory, serves lines from buffer = fast
- **`read_to_string()`** = reads everything in 1 call = fastest
- **`BufWriter`** = same idea for output
- The 8KB buffer **auto-refills** — you never manage it, just keep reading
