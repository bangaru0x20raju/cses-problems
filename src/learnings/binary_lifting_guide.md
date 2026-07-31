# Binary Lifting — Complete Guide

## 1. What is Binary Lifting?

Binary lifting is a powerful technique used primarily on trees (and sometimes graphs) to make "jumps" of size 2^i instead of taking steps one by one. This allows us to jump k steps up a tree in O(log k) time instead of O(k) time.

The name comes from:
- **Binary**: Using powers of 2 (base 2 representation).
- **Lifting**: "Lifting" a node up the tree towards the root.

### The Core Intuition

Any number k can be represented as a sum of powers of 2 (its binary representation).
For example, to jump k = 13 steps up:
13 = 8 + 4 + 1 = 2^3 + 2^2 + 2^0$

Instead of taking 13 individual steps, we can take:
1. A jump of size 8
2. A jump of size 4
3. A jump of size 1

If we precompute the ancestors at distances 1, 2, 4, 8, 16... for every node, we can make any jump of size k in O(log k) steps!

---

## 2. Precomputation (The `up` table)

We build a 2D array, usually called `up[u][i]`, which stores the **2^i-th ancestor** of node `u`.

### The Recurrence Relation

The brilliant part of binary lifting is how we build this table efficiently:
To find the 2^i-th ancestor of `u`, we can first find the 2^{i-1}-th ancestor of `u` (let's call it `mid`), and then find the 2^{i-1}-th ancestor of `mid`.

Mathematically:
`up[u][i] = up[ up[u][i-1] ][ i-1 ]`

### C++ Setup and Precomputation

```cpp
#include <bits/stdc++.h>
using namespace std;

const int MAXN = 200005;
const int LOG = 20; // ceil(log2(200000)) is around 18, 20 is safe

int up[MAXN][LOG]; // up[u][i] is the 2^i-th ancestor of u
int depth[MAXN];   // depth of node from root
vector<int> adj[MAXN];

// DFS to compute up[u][0] (direct parent) and depth
void dfs(int u, int p, int d) {
    up[u][0] = p;
    depth[u] = d;
    
    for (int v : adj[u]) {
        if (v != p) {
            dfs(v, u, d + 1);
        }
    }
}

// Build the binary lifting table
void build_lifting(int n, int root) {
    // 1. Run DFS to get direct parents (up[u][0]) and depths
    dfs(root, root, 0); // Root's parent is itself
    
    // 2. Fill the table using DP
    for (int i = 1; i < LOG; i++) {
        for (int u = 1; u <= n; u++) {
            // The 2^i-th ancestor is the 2^{i-1}-th ancestor of the 2^{i-1}-th ancestor
            up[u][i] = up[ up[u][i-1] ][ i-1 ];
        }
    }
}
```

---

## 3. Application 1: Finding the k-th Ancestor

Given a node u and an integer k, find the node that is k steps above u in the tree.

### The Algorithm
Iterate through the bits of k. If the i-th bit of k is set (i.e., 1), we jump to the 2^i-th ancestor.

### C++ Implementation

```cpp
int get_kth_ancestor(int u, int k) {
    for (int i = 0; i < LOG; i++) {
        if (k & (1 << i)) { // If the i-th bit of k is 1
            u = up[u][i];
        }
    }
    return u;
}
```
**Time Complexity**: O(log k) per query.

---

## 4. Application 2: Lowest Common Ancestor (LCA)

The most famous use of binary lifting is finding the Lowest Common Ancestor of two nodes u and v in O(log N) time.

### The Algorithm

1. **Level them up:** If u and v are at different depths, move the deeper node up until they are at the exact same depth.
2. **Check if they met:** If u == v now, they were on the same branch, and we are done. LCA is u.
3. **Jump together:** Now both nodes are at the same depth but might be different nodes. We want to jump them up together as high as possible *without* reaching their common ancestor.
   - We loop i from LOG-1 down to 0.
   - If `up[u][i] != up[v][i]`, it means jumping 2^i steps does *not* overshoot the LCA. So we jump both: `u = up[u][i]`, `v = up[v][i]`.
4. **Final step:** After the loop, u and v will be exactly one step below the LCA. The LCA is simply `up[u][0]` (the direct parent).

### C++ Implementation

```cpp
int get_lca(int u, int v) {
    // 1. Ensure u is the deeper node
    if (depth[u] < depth[v]) {
        swap(u, v);
    }
    
    // 2. Level them up (jump u up by depth[u] - depth[v] steps)
    int k = depth[u] - depth[v];
    for (int i = 0; i < LOG; i++) {
        if (k & (1 << i)) {
            u = up[u][i];
        }
    }
    
    // 3. If they met, we found the LCA
    if (u == v) return u;
    
    // 4. Jump together as high as possible without meeting
    for (int i = LOG - 1; i >= 0; i--) {
        if (up[u][i] != up[v][i]) {
            u = up[u][i];
            v = up[v][i];
        }
    }
    
    // 5. The LCA is the parent of u (or v)
    return up[u][0];
}
```
**Time Complexity:** O(log N) per query.

---

## 5. Extensions of Binary Lifting

Binary lifting isn't just for finding ancestors. You can augment the `up` table to compute functions along a path.

### Min/Max/Sum on a Path
Suppose edges have weights and you want to find the maximum edge weight on the path from u to v.
You can maintain `max_val[u][i]` which stores the max edge weight on the path from u to its 2^i-th ancestor.

```cpp
max_val[u][i] = max( max_val[u][i-1], max_val[ up[u][i-1] ][ i-1 ] );
```
When answering queries, you jump up towards the LCA and accumulate the max values along the way!

---

## 6. Complexity Summary

| Operation | Complexity | Description |
|---|---|---|
| **Space** | O(N log N) | Table size is N * log(N) |
| **Precomputation** | O(N log N) | Building the `up` table |
| **k-th Ancestor Query**| O(log N) | At most log N jumps |
| **LCA Query** | O(log N) | At most log N jumps |

## 7. Mental Model

Think of binary lifting like having an **express elevator system** in a skyscraper:
- Instead of taking the stairs one by one (O(N)).
- You have elevators that skip 1 floor, 2 floors, 4 floors, 8 floors, etc.
- To go up 13 floors, you take the 8-floor elevator, then the 4-floor elevator, then the 1-floor elevator.
- The DP recurrence `up[u][i] = up[up[u][i-1]][i-1]` is just saying: "taking the 8-floor elevator is the same as taking the 4-floor elevator twice."
