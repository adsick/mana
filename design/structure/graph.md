# 2024-10-05
```rust
pub struct Graph<V, E = ()> {
    nodes: Vec<Node<V, E>>,
}


pub struct Node<V, E> {
    value: V,
    next: Vec<(E, NodeId)>,
    prev: Vec<NodeId>,
}
```
Currently I have a basic "flat" graph implemented, but I thought that it would be useful to have some kind hierarchical graph structure. A Graph of graphs so to speak. Don't know what the name of that is, Copilot proposed `GraphSet` but idk.
