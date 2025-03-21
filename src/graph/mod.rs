mod builder;
mod node;
#[cfg(test)]
mod test;

pub use builder::GraphBuilder;

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

pub use node::{Node, NodeId};

// todo fix: it's graph structure is not consistent right now.
// the 0'th node is the root node (usually)

// consider implementing 'relative' indexing for subgraphs
#[derive(PartialEq)]
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
            .iter()
            .map(move |(e, id)| (id, e, &self.nodes[*id]))
    }
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

impl<V, E> std::fmt::Debug for Graph<V, E>
where
    V: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            let value = node.value();
            let edges = node.edges();
            writeln!(f);
            if !edges.is_empty() {
                writeln!(f, "{i} ({value:?}) => {{");
                for ((edge, id)) in node.edges().iter() {
                    writeln!(f, "  {id}: {:?},", edge);
                }
                write!(f, "}}");
            } else {
                write!(f, "{i} ({value:?}),");
            }
        }
        Ok(())
    }
}
