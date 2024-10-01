use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use super::{Graph, Node, NodeId};

// this type is used for more ergonomic graph construction, but I doubt how good it is to use in prod
// the idea is to keep a pointer to a certain node
pub struct GraphBuilder<'g, V, E> {
    graph: Rc<RefCell<&'g mut Graph<V, E>>>,
    node: NodeId,
}

impl<'g, V, E> GraphBuilder<'g, V, E> {
    pub fn new(graph: &'g mut Graph<V, E>, node: NodeId) -> Self {
        Self {
            graph: Rc::new(RefCell::new(graph)),
            node,
        }
    }

    /// add child to this node, can be chained
    pub fn add(&self, edge: E, value: V) -> Self {
        let id = self
            .graph
            .borrow_mut()
            .add_to(self.node, edge, Node::new(value))
            .unwrap();

        GraphBuilder {
            graph: self.graph.clone(),
            node: id,
        }
    }

    pub fn id(&self) -> NodeId {
        self.node
    }

    // ref to current node
    pub fn node(&self) -> Ref<Node<V, E>> {
        let gr = self.graph.borrow();
        Ref::map(gr, |s| &s.nodes[self.node])
    }

    // mutable ref to current node
    pub fn node_mut(&self) -> RefMut<Node<V, E>> {
        let gr = self.graph.borrow_mut();
        RefMut::map(gr, |g| &mut g[self.node])
    }

    // ref to current node's value
    pub fn value(&self) -> Ref<V> {
        let node = self.node();
        Ref::map(node, |n| n.value())
    }

    // ref to current node's value
    pub fn value_mut(&self) -> RefMut<V> {
        let node = self.node_mut();
        RefMut::map(node, |n| n.value_mut())
    }

    // ref to current node's edges
    pub fn edges(&self) -> Ref<[(E, NodeId)]> {
        let node = self.node();
        Ref::map(node, |n| n.edges())
    }

    // ref to current node's value
    pub fn edges_mut(&self) -> RefMut<[(E, NodeId)]> {
        let node = self.node_mut();
        RefMut::map(node, |n| n.edges_mut())
    }

    pub fn looping(&self, edge: E) -> Self {
        self.link_id(self.node, edge);
        self.clone()
    }

    pub fn link_id(&self, id: NodeId, edge: E) -> Self {
        self.graph.borrow_mut().link(self.node, id, edge).unwrap();
        self.clone()
    }

    pub fn link(&self, node: &Self, edge: E) -> Self {
        self.graph
            .borrow_mut()
            .link(self.node, node.id(), edge)
            .unwrap();
        self.clone()
    }
}

impl<'g, V, E> Clone for GraphBuilder<'g, V, E> {
    fn clone(&self) -> Self {
        Self {
            graph: self.graph.clone(),
            node: self.node,
        }
    }
}

// todo: add api for getting specific edges?

// pub struct EdgeBuilder<'n, V, E> {
//     node: &'n mut Node<V, E>,
// }

// impl<'n, V, E> EdgeBuilder<'n, V, E> {
//     pub fn new(node: &'n mut Node<V, E>) -> Self {
//         Self { node }
//     }
//     pub fn val(self, value: E) {}
// }

// impl<'n, V, E> Drop for EdgeBuilder<'n, V, E>{
//     fn drop(&mut self) {
//         self.node
//     }
// }
