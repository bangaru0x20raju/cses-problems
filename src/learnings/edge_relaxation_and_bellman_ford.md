# Edge Relaxation and the Bellman-Ford Algorithm

## What is Edge Relaxation?

"Relaxing an edge" means: **"Can I find a shorter path to a node by going through this edge?"**

### The Core Operation

For an edge `u → v` with weight `w`:

```
if dist[u] + w < dist[v]:
    dist[v] = dist[u] + w     ← this update IS the "relaxation"
```

You're checking: *"Is reaching `v` through `u` cheaper than the current best known way to reach `v`?"* If yes, update it.

---

## Why Is It Called "Relaxing"?

Think of `dist[v]` as a **tight rubber band** stretched to some large value. Each time you find a shorter path, you "relax" (loosen) it to a smaller value. The distance gets more and more relaxed (closer to the true shortest distance) with each update.

```
Initial:    dist[v] = ∞        (fully stretched, very tight)
After 1st:  dist[v] = 20       (relaxed a bit)
After 2nd:  dist[v] = 15       (relaxed more)
After 3rd:  dist[v] = 12       (relaxed to final shortest distance)
```

---

## Visualizing Relaxation on a Graph

```
Start: dist = [0, ∞, ∞, ∞]
       Node:   1  2  3  4

Edges: 1→2 (cost 10), 1→3 (cost 5), 3→2 (cost 3), 2→4 (cost 7)
```

### Round 1 — Relax all edges:
```
Edge 1→2:  dist[1] + 10 = 10  <  dist[2] = ∞   → dist[2] = 10  ✅ relaxed!
Edge 1→3:  dist[1] + 5  = 5   <  dist[3] = ∞   → dist[3] = 5   ✅ relaxed!
Edge 3→2:  dist[3] + 3  = 8   <  dist[2] = 10  → dist[2] = 8   ✅ relaxed again!
Edge 2→4:  dist[2] + 7  = 15  <  dist[4] = ∞   → dist[4] = 15  ✅ relaxed!

After Round 1: dist = [0, 8, 5, 15]
```

### Round 2 — Relax all edges again:
```
Edge 1→2:  dist[1] + 10 = 10  >  dist[2] = 8   → no change
Edge 1→3:  dist[1] + 5  = 5   =  dist[3] = 5   → no change
Edge 3→2:  dist[3] + 3  = 8   =  dist[2] = 8   → no change
Edge 2→4:  dist[2] + 7  = 15  =  dist[4] = 15  → no change

After Round 2: dist = [0, 8, 5, 15]  ← nothing changed, distances are final!
```

---

## The Bellman-Ford Algorithm

Bellman-Ford = **Relax ALL edges, N-1 times.**

```rust
// Bellman-Ford algorithm
let mut dist = vec![i64::MAX; n + 1];
dist[1] = 0;

for _round in 0..n-1 {             // Repeat N-1 times
    for (u, v, w) in &edges {      // Go through EVERY edge
        if dist[*u] != i64::MAX && dist[*u] + w < dist[*v] {
            dist[*v] = dist[*u] + w;   // ← THIS is edge relaxation
        }
    }
}
```

---

## Why N-1 Rounds?

The shortest path between any two nodes can have **at most N-1 edges** (visiting all N nodes without repeating). Each round guarantees at least one more node gets its correct shortest distance:

```
Round 1: Nodes reachable via 1 edge from source are correct
Round 2: Nodes reachable via 2 edges from source are correct
Round 3: Nodes reachable via 3 edges from source are correct
...
Round N-1: ALL reachable nodes have their correct shortest distance
```

### Why this works:

Consider the shortest path from source to some node `t`:
```
source → a → b → c → t   (4 edges)
```
- After Round 1: `dist[a]` is correct (1 edge from source)
- After Round 2: `dist[b]` is correct (uses the now-correct `dist[a]`)
- After Round 3: `dist[c]` is correct (uses the now-correct `dist[b]`)
- After Round 4: `dist[t]` is correct (uses the now-correct `dist[c]`)

In the worst case, the longest shortest path has N-1 edges, so N-1 rounds is enough.

---

## Detecting Negative Cycles

If you do one **extra round (Round N)** and any distance still gets relaxed, it means there's a **negative cycle** — a loop where the total weight is negative, so you can keep going around it to reduce the distance infinitely.

```rust
// After N-1 rounds of normal Bellman-Ford...

// Round N — check for negative cycles
let mut has_negative_cycle = false;
for (u, v, w) in &edges {
    if dist[*u] != i64::MAX && dist[*u] + w < dist[*v] {
        has_negative_cycle = true;
        break;
    }
}

if has_negative_cycle {
    println!("Negative cycle detected!");
}
```

### Why does this work?

After N-1 rounds, all shortest distances should be finalized (if no negative cycle exists). If Round N still relaxes an edge, it means some path keeps getting shorter — which is only possible if there's a cycle with negative total weight.

```
Example negative cycle: A →(2) B →(3) C →(-10) A
Total weight: 2 + 3 + (-10) = -5

Each time you go around: distance decreases by 5
Round 1:  dist[A] = 0
Round 2:  dist[A] = -5   (went around once)
Round 3:  dist[A] = -10  (went around twice)
...keeps decreasing forever!
```

---

## Early Termination Optimization

If no edge gets relaxed during a round, all distances are final — you can stop early:

```rust
for _round in 0..n-1 {
    let mut any_relaxed = false;
    for (u, v, w) in &edges {
        if dist[*u] != i64::MAX && dist[*u] + w < dist[*v] {
            dist[*v] = dist[*u] + w;
            any_relaxed = true;
        }
    }
    if !any_relaxed {
        break;  // All distances are final, no need for more rounds
    }
}
```

---

## Bellman-Ford vs Dijkstra

Both algorithms use edge relaxation, but in different ways:

| | Bellman-Ford | Dijkstra |
| :--- | :--- | :--- |
| How it relaxes | Relaxes **all edges**, N-1 times | Relaxes only **neighbors** of the closest unvisited node |
| Negative weights | ✅ Handles them correctly | ❌ Breaks with negative weights |
| Negative cycle detection | ✅ Yes (extra round N) | ❌ No |
| Time complexity | O(V × E) — slower | O((V+E) log V) — faster |
| When to use | Graph has negative weights or need cycle detection | Graph has only non-negative weights |

### Why Dijkstra breaks with negative weights:

```
Dijkstra processes node 2 first (closest):
    1 →(1) 2
    1 →(10) 3 →(-20) 2

Dijkstra sets dist[2] = 1 and never revisits it.
But the actual shortest path is: 1 → 3 → 2 = 10 + (-20) = -10
```

Dijkstra assumes once a node is processed, its distance is final. Negative edges violate this assumption.

---

## Full Bellman-Ford Implementation in Rust

```rust
fn bellman_ford(n: usize, edges: &Vec<(usize, usize, i64)>, source: usize) -> (Vec<i64>, bool) {
    let mut dist = vec![i64::MAX; n + 1];
    dist[source] = 0;

    // N-1 rounds of relaxation
    for _round in 0..n - 1 {
        let mut any_relaxed = false;
        for &(u, v, w) in edges {
            if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                any_relaxed = true;
            }
        }
        if !any_relaxed {
            break; // Early termination
        }
    }

    // Round N: check for negative cycles
    let mut has_negative_cycle = false;
    for &(u, v, w) in edges {
        if dist[u] != i64::MAX && dist[u] + w < dist[v] {
            has_negative_cycle = true;
            break;
        }
    }

    (dist, has_negative_cycle)
}
```

---

## TL;DR

- **Edge relaxation** = "Can I reach `v` cheaper by going through `u`?" If yes, update `dist[v]`.
- **Bellman-Ford** = Relax all edges, repeat N-1 times.
- **N-1 rounds** because the longest shortest path has at most N-1 edges.
- **Round N** detects negative cycles (if any edge still relaxes, there's a negative cycle).
- Use **Bellman-Ford** when edges can be negative. Use **Dijkstra** when all edges are non-negative (it's faster).
