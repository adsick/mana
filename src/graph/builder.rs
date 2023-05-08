use std::{cell::RefCell, rc::Rc};

use super::{Graph, Node, NodeId};

// this type is used for more ergonomic graph construction, but I doubt how good it is to use in prod
pub struct GraphBuilder<'g, V, E> {
    graph: Rc<RefCell<&'g mut Graph<V, E>>>,
    node: NodeId,
}

// impl<'g, T> Deref for GraphBuilder<'g, T>{
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         self.graph.borrow().nodes[self.node].value()
//     }
// }

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
