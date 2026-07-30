# Topological Sorting — Complete Guide

## 1. What Is Topological Sorting?

A **topological sort** of a directed acyclic graph (DAG) is a **linear ordering of its vertices** such that for every directed edge `u → v`, vertex `u` comes **before** vertex `v` in the ordering.

> [!IMPORTANT]
> Topological sorting is only possible on **Directed Acyclic Graphs (DAGs)**. If the graph has a cycle, no valid topological order exists.

### Real-World Analogies
| Scenario | Vertices | Edges (u → v) |
|---|---|---|
| University courses | Courses | Prerequisites |
| Build systems (Make) | Compilation targets | Dependencies |
| Task scheduling | Tasks | "must finish before" |
| Package managers | Packages | Dependencies |

### Key Properties
- A DAG can have **multiple valid** topological orderings.
- A DAG with `n` vertices has a topological order of length `n`.
- The **first vertex** in any topological order has **in-degree 0** (no prerequisites).
- The **last vertex** has **out-degree 0** (nothing depends on it).

---

## 2. Technique 1 — Kahn's Algorithm (BFS / In-degree Method)

This is the most intuitive approach: **repeatedly remove vertices with no incoming edges**.

### The Idea
1. Compute the **in-degree** (number of incoming edges) of every vertex.
2. Push all vertices with in-degree `0` into a queue.
3. While the queue is not empty:
   - Pop a vertex `u`, add it to the result.
   - For each neighbor `v` of `u`, decrement `in_degree[v]`.
   - If `in_degree[v]` becomes `0`, push `v` into the queue.
4. If the result contains all `n` vertices → valid topological order.  
   Otherwise → the graph has a **cycle**.

### Visualization

```
Graph:  5 → 0,  5 → 2,  4 → 0,  4 → 1,  2 → 3,  3 → 1

Step-by-step:
  In-degrees:  0:2  1:2  2:1  3:1  4:0  5:0
  
  Queue: [4, 5]          Result: []
  Pop 4 → Result: [4]    Update: 0:1, 1:1
  Pop 5 → Result: [4,5]  Update: 0:0, 2:0
  Pop 0 → Result: [4,5,0]
  Pop 2 → Result: [4,5,0,2]  Update: 3:0
  Pop 3 → Result: [4,5,0,2,3]  Update: 1:0
  Pop 1 → Result: [4,5,0,2,3,1]  ✅ All 6 vertices included
```

### C++ Implementation

```cpp
#include <bits/stdc++.h>
using namespace std;

// Returns topological order, or empty vector if cycle exists
vector<int> kahns_toposort(int n, vector<vector<int>>& adj) {
    vector<int> in_degree(n, 0);
    
    // Step 1: Compute in-degrees
    for (int u = 0; u < n; u++)
        for (int v : adj[u])
            in_degree[v]++;
    
    // Step 2: Enqueue all vertices with in-degree 0
    queue<int> q;
    for (int i = 0; i < n; i++)
        if (in_degree[i] == 0)
            q.push(i);
    
    // Step 3: Process
    vector<int> topo_order;
    while (!q.empty()) {
        int u = q.front();
        q.pop();
        topo_order.push_back(u);
        
        for (int v : adj[u]) {
            in_degree[v]--;
            if (in_degree[v] == 0)
                q.push(v);
        }
    }
    
    // Step 4: Check for cycle
    if ((int)topo_order.size() != n)
        return {};  // Cycle detected!
    
    return topo_order;
}
```

### Complexity
- **Time:** `O(V + E)` — each vertex and edge is processed once
- **Space:** `O(V + E)` — adjacency list + in-degree array + queue

> [!TIP]
> **Cycle Detection Trick:** If `topo_order.size() < n` after Kahn's finishes, the graph has a cycle. The vertices NOT in `topo_order` are involved in or blocked by the cycle.

---

## 3. Technique 2 — DFS-Based Topological Sort

Use **Depth-First Search** and add vertices to the result in **reverse post-order** (i.e., add a vertex to the front after all its descendants are fully explored).

### The Idea
1. Maintain a `visited` array and a `stack` (or a list you reverse at the end).
2. For each unvisited vertex, run DFS.
3. In DFS, after visiting **all** neighbors of `u`, push `u` onto the stack.
4. The stack (read top-to-bottom) gives the topological order.

### Why It Works
When `u → v` exists and we finish DFS on `u`, vertex `v` was already fully explored (pushed to stack) before `u`. So `u` sits above `v` in the stack — exactly what we need.

### C++ Implementation

```cpp
#include <bits/stdc++.h>
using namespace std;

enum Color { WHITE, GRAY, BLACK };

bool dfs(int u, vector<vector<int>>& adj, vector<Color>& color, vector<int>& order) {
    color[u] = GRAY;  // Currently being explored
    
    for (int v : adj[u]) {
        if (color[v] == GRAY)
            return false;  // Back edge → Cycle detected!
        if (color[v] == WHITE) {
            if (!dfs(v, adj, color, order))
                return false;
        }
    }
    
    color[u] = BLACK;  // Fully explored
    order.push_back(u);  // Post-order: add AFTER all descendants
    return true;
}

vector<int> dfs_toposort(int n, vector<vector<int>>& adj) {
    vector<Color> color(n, WHITE);
    vector<int> order;
    
    for (int i = 0; i < n; i++) {
        if (color[i] == WHITE) {
            if (!dfs(i, adj, color, order))
                return {};  // Cycle!
        }
    }
    
    reverse(order.begin(), order.end());  // Reverse post-order
    return order;
}
```

### The Three Colors (Important for Cycle Detection)
| Color | Meaning |
|---|---|
| **WHITE** | Not yet visited |
| **GRAY** | Currently in the DFS call stack (being explored) |
| **BLACK** | Fully explored, all descendants processed |

> [!IMPORTANT]
> **Cycle Detection:** If during DFS from `u`, we encounter a **GRAY** vertex `v`, then `v → ... → u → v` forms a cycle (a **back edge**). This is the standard method to detect cycles in directed graphs.

---

## 4. Kahn's vs DFS — When to Use Which?

| Feature | Kahn's (BFS) | DFS |
|---|---|---|
| **Cycle detection** | ✅ Easy (count processed vertices) | ✅ Easy (detect back edges) |
| **Lexicographically smallest order** | ✅ Use a **min-heap** instead of queue | ❌ Not straightforward |
| **Counting topological orders** | ❌ Hard | ✅ Via DP on DFS |
| **Longest path in DAG** | ✅ Process in topo order | ✅ DP in reverse topo order |
| **Intuitive for beginners** | ✅ Very intuitive | Moderate |
| **Implementation** | Iterative | Recursive (or explicit stack) |

---

## 5. Advanced Techniques & Patterns

### 5.1 Lexicographically Smallest Topological Order

Replace the `queue` in Kahn's with a **min-priority queue (min-heap)**:

```cpp
vector<int> lex_smallest_toposort(int n, vector<vector<int>>& adj) {
    vector<int> in_degree(n, 0);
    for (int u = 0; u < n; u++)
        for (int v : adj[u])
            in_degree[v]++;
    
    priority_queue<int, vector<int>, greater<int>> pq;  // Min-heap!
    for (int i = 0; i < n; i++)
        if (in_degree[i] == 0)
            pq.push(i);
    
    vector<int> order;
    while (!pq.empty()) {
        int u = pq.top();
        pq.pop();
        order.push_back(u);
        
        for (int v : adj[u]) {
            if (--in_degree[v] == 0)
                pq.push(v);
        }
    }
    return order;
}
```

**Time:** `O((V + E) log V)` due to heap operations.

---

### 5.2 Longest Path in a DAG (DP + Topo Sort)

One of the most powerful applications. Process vertices in topological order and relax edges:

```cpp
// Finds the longest path from source 'src' in a DAG
vector<int> longest_path_dag(int n, vector<vector<pair<int,int>>>& adj, int src) {
    // First, get topological order (Kahn's on unweighted adj)
    vector<int> in_deg(n, 0);
    for (int u = 0; u < n; u++)
        for (auto [v, w] : adj[u])
            in_deg[v]++;
    
    queue<int> q;
    for (int i = 0; i < n; i++)
        if (in_deg[i] == 0) q.push(i);
    
    vector<int> topo;
    while (!q.empty()) {
        int u = q.front(); q.pop();
        topo.push_back(u);
        for (auto [v, w] : adj[u])
            if (--in_deg[v] == 0) q.push(v);
    }
    
    // DP: process in topological order
    vector<int> dist(n, INT_MIN);
    dist[src] = 0;
    
    for (int u : topo) {
        if (dist[u] == INT_MIN) continue;
        for (auto [v, w] : adj[u]) {
            dist[v] = max(dist[v], dist[u] + w);
        }
    }
    
    return dist;
}
```

> [!TIP]
> The shortest path in a DAG works identically — just initialize `dist` to `INT_MAX` and use `min` instead of `max`.

---

### 5.3 Counting Paths in a DAG

Process in topological order, accumulating path counts:

```cpp
// Count number of paths from 'src' to each vertex
vector<long long> count_paths(int n, vector<vector<int>>& adj, int src) {
    // Get topological order first (Kahn's)
    // ... (same as above) ...
    
    vector<long long> paths(n, 0);
    paths[src] = 1;
    
    for (int u : topo) {
        for (int v : adj[u]) {
            paths[v] += paths[u];
        }
    }
    
    return paths;
}
```

---

### 5.4 Checking if Topological Order Is Unique (Hamiltonian Path in DAG)

A topological order is **unique** if and only if there is a **Hamiltonian path** in the DAG — meaning consecutive vertices in the ordering are connected by edges.

**Simple check with Kahn's:** The topological order is unique **if and only if the queue never contains more than one element** at any point.

```cpp
bool is_unique_toposort(int n, vector<vector<int>>& adj) {
    vector<int> in_degree(n, 0);
    for (int u = 0; u < n; u++)
        for (int v : adj[u])
            in_degree[v]++;
    
    queue<int> q;
    for (int i = 0; i < n; i++)
        if (in_degree[i] == 0) q.push(i);
    
    while (!q.empty()) {
        if (q.size() > 1) return false;  // Not unique!
        int u = q.front(); q.pop();
        for (int v : adj[u])
            if (--in_degree[v] == 0) q.push(v);
    }
    return true;
}
```

---

### 5.5 All Topological Orderings (Backtracking)

Generate every valid ordering — useful for small graphs:

```cpp
void all_toposorts(int n, vector<vector<int>>& adj, vector<int>& in_degree,
                   vector<bool>& visited, vector<int>& current,
                   vector<vector<int>>& results) {
    if ((int)current.size() == n) {
        results.push_back(current);
        return;
    }
    
    for (int u = 0; u < n; u++) {
        if (!visited[u] && in_degree[u] == 0) {
            visited[u] = true;
            current.push_back(u);
            for (int v : adj[u]) in_degree[v]--;
            
            all_toposorts(n, adj, in_degree, visited, current, results);
            
            // Backtrack
            for (int v : adj[u]) in_degree[v]++;
            current.pop_back();
            visited[u] = false;
        }
    }
}
```

> [!WARNING]
> The number of topological orderings can be **exponential**. Only use this for small `n`.

---

## 6. Classic CSES Problems Using Topological Sort

| Problem | Technique |
|---|---|
| **Course Schedule** | Basic Kahn's / DFS toposort |
| **Longest Flight Route** | DP on DAG (longest path) |
| **Shortest Routes** in DAG | DP on DAG (shortest path) |
| **Game Routes** | Count paths in DAG |
| **Coin Combinations** | Sometimes modeled as DAG DP |

---

## 7. Common Pitfalls

> [!CAUTION]
> 1. **Forgetting cycle detection** — Always check! A graph that "should be" a DAG might have a cycle due to bad input or bugs.
> 2. **1-indexed vs 0-indexed vertices** — Be consistent. CSES uses 1-indexed.
> 3. **Not reversing in DFS** — The DFS post-order gives **reverse** topological order. You must reverse it!
> 4. **Using toposort on undirected graphs** — Topological sort is only defined for **directed** graphs.

---

## 8. Mental Model / Cheat Sheet

```
┌─────────────────────────────────────────────────────┐
│              TOPOLOGICAL SORT DECISION TREE         │
│                                                     │
│  Need any valid topo order?                         │
│    → Kahn's (BFS) or DFS — both O(V+E)              │
│                                                     │
│  Need lexicographically smallest?                   │
│    → Kahn's with min-heap — O((V+E) log V)          │
│                                                     │
│  Need to detect cycle?                              │
│    → DFS with 3 colors (WHITE/GRAY/BLACK)           │
│    → Or Kahn's: cycle exists if result.size() < n   │
│                                                     │
│  Need longest/shortest path in DAG?                 │
│    → Topo sort + DP relaxation — O(V+E)             │
│                                                     │
│  Need to count paths?                               │
│    → Topo sort + DP accumulation — O(V+E)           │
│                                                     │
│  Need ALL valid orderings?                          │
│    → Backtracking (exponential, small n only)       │
│                                                     │
│  Is topo order unique?                              │
│    → Kahn's: queue never has > 1 element            │
└─────────────────────────────────────────────────────┘
```

---

## 9. Summary

| Aspect | Details |
|---|---|
| **Precondition** | Graph must be a **DAG** |
| **Two main algorithms** | Kahn's (BFS, in-degree) and DFS (reverse post-order) |
| **Time complexity** | `O(V + E)` for both |
| **Cycle detection** | Built into both algorithms |
| **Key applications** | Scheduling, dependency resolution, DP on DAGs, shortest/longest paths |
