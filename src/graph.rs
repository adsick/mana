mod builder;
mod node;

pub use builder::GraphBuilder;

use std::ops::{Index, IndexMut};

pub use node::{Node, NodeId};

// todo fix: it's graph structure is not consistent right now - Node Id's can not be used for indexing root children and inner nodes at the same time.
// the 0'th node is the root node (usually)

// consider implementing 'relative' indexing for subgraphs
#[derive(Debug, PartialEq)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T: Default> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
    }
}

impl<T> Graph<T> {
    pub fn new(root: T) -> Self {
        Self {
            nodes: vec![Node::new(root)],
        }
    }

    // todo: refine
    pub fn build(&mut self) -> GraphBuilder<'_, T> {
        GraphBuilder::new(self, 0)
    }

    // not sure about this api
    /// adds a node, returns `GraphBuilder`
    pub fn add(&mut self, node: T) -> GraphBuilder<'_, T> {
        GraphBuilder::new(self, 0).add(node)
    }

    pub fn add_to(&mut self, parent: NodeId, node: Node<T>) -> Option<NodeId> {
        self.nodes.push(node);

        let id = self.nodes.len() - 1;
        self.link(parent, id).ok()?;
        return Some(id);
    }

    pub fn link(&mut self, parent: NodeId, child: NodeId) -> Result<(), ()> {
        if let Some(ref mut node) = self.nodes.get_mut(parent) {
            node.add_child(child);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }

    pub fn get_children(&self, id: NodeId) -> impl Iterator<Item = (&NodeId, &Node<T>)> {
        self.nodes[id].children().map(move |id| (id, &self.nodes[*id]))
    }

    pub fn is_leaf_node(&self, id: NodeId) -> bool {
        self.nodes[id].children().next().is_none()
    }
}

impl<T> Index<NodeId> for Graph<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index]
    }
}
impl<T> IndexMut<NodeId> for Graph<T> {
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

        root.add('a').add('b');

        let node_d = root.add('c').add('d');

        let node_e = node_d.add('e');
        let node_f = node_d.add('f');
        let node_g = node_d.add('g');

        node_e.looping();

        node_f.link(&node_g);
        node_g.link(&node_f);

        dbg!(&graph);

        // now let's create the same graph without builder

        let mut root = Node::new('\0'); // 0
        let mut a = Node::new('a'); // 1
        root.add_child(1);

        let b = Node::new('b'); // 2
        a.add_child(2);

        let mut c = Node::new('c'); // 3
        root.add_child(3);

        let mut d = Node::new('d'); // 4
        c.add_child(4);

        let mut e = Node::new('e'); // 5
        let mut f = Node::new('f'); // 6
        let mut g = Node::new('g'); // 7

        d.add_child(5);
        d.add_child(6);
        d.add_child(7);

        e.add_child(5);

        f.add_child(7);
        g.add_child(6);

        let expected = Graph {
            nodes: vec![root, a, b, c, d, e, f, g],
        };

        assert_eq!(graph, expected);
    }
}
