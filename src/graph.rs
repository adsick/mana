mod builder;
mod node;

pub use builder::GraphBuilder;

use std::ops::{Index, IndexMut};

pub use node::{Node, NodeId};

// todo fix: it's graph structure is not consistent right now.
// the 0'th node is the root node (usually)

// consider implementing 'relative' indexing for subgraphs
#[derive(Debug, PartialEq)]
pub struct Graph<V, E = ()> {
    nodes: Vec<Node<V, E>>,
}

impl<V: Default, E: Default> Default for Graph<V, E> {
    fn default() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
    }
}

impl<V, E> Graph<V, E> {
    pub fn new(root: V) -> Self {
        Self {
            nodes: vec![Node::new(root)],
        }
    }

    // todo: refine
    pub fn build(&mut self) -> GraphBuilder<'_, V, E> {
        GraphBuilder::new(self, 0)
    }

    // not sure about this api
    /// adds a node, returns `GraphBuilder`
    pub fn add(&mut self, edge: E, value: V) -> GraphBuilder<'_, V, E> {
        GraphBuilder::new(self, 0).add(edge, value)
    }

    pub fn add_to(&mut self, parent: NodeId, edge: E, node: Node<V, E>) -> Option<NodeId> {
        self.nodes.push(node);

        let id = self.nodes.len() - 1;
        self.link(parent, id, edge).ok()?;
        return Some(id);
    }

    pub fn link(&mut self, parent: NodeId, child: NodeId, val: E) -> Result<(), ()> {
        if let Some(ref mut node) = self.nodes.get_mut(parent) {
            node.add_edge(child, val);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node<V, E>> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<V, E>> {
        self.nodes.get_mut(id)
    }

    /// returns an iterator of edges (node id, associated value, child node)
    pub fn edges(&self, id: NodeId) -> impl Iterator<Item = (&NodeId, &E, &Node<V, E>)> {
        self.nodes[id]
            .edges()
            .map(move |(e, id)| (id, e, &self.nodes[*id]))
    }

    // better just use graph[id].leaf()
    // pub fn is_leaf_node(&self, id: NodeId) -> bool {
    //     self.nodes[id].leaf()
    // }
}

impl<V, E> Index<NodeId> for Graph<V, E> {
    type Output = Node<V, E>;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index]
    }
}
impl<V, E> IndexMut<NodeId> for Graph<V, E> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn graph() {
        let mut graph: Graph<char> = Graph::new('\0');

        let root = graph.build();

        root.add((), 'a').add((), 'b');

        let node_d = root.add((), 'c').add((), 'd');

        let node_e = node_d.add((), 'e');
        let node_f = node_d.add((), 'f');
        let node_g = node_d.add((), 'g');

        node_e.looping(());

        node_f.link(&node_g, ());
        node_g.link(&node_f, ());

        dbg!(&graph);

        // now let's create the same graph without builder

        let mut root = Node::new('\0'); // 0
        let mut a = Node::new('a'); // 1
        root.add_edge(1, ());

        let b = Node::new('b'); // 2
        a.add_edge(2, ());

        let mut c = Node::new('c'); // 3
        root.add_edge(3, ());

        let mut d = Node::new('d'); // 4
        c.add_edge(4, ());

        let mut e = Node::new('e'); // 5
        let mut f = Node::new('f'); // 6
        let mut g = Node::new('g'); // 7

        d.add_edge(5, ());
        d.add_edge(6, ());
        d.add_edge(7, ());

        e.add_edge(5, ());

        f.add_edge(7, ());
        g.add_edge(6, ());

        let expected = Graph {
            nodes: vec![root, a, b, c, d, e, f, g],
        };

        assert_eq!(graph, expected);
    }
}
