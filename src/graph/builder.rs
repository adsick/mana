use std::{cell::RefCell, rc::Rc};

use super::{Graph, Node, NodeId};

pub struct GraphBuilder<'g, T> {
    graph: Rc<RefCell<&'g mut Graph<T>>>,
    node: NodeId,
}

// impl<'g, T> Deref for GraphBuilder<'g, T>{
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         self.graph.borrow().nodes[self.node].value()
//     }
// }

impl<'g, T> GraphBuilder<'g, T> {
    pub fn new(graph: &'g mut Graph<T>, node: NodeId) -> Self {
        Self {
            graph: Rc::new(RefCell::new(graph)),
            node,
        }
    }

    /// add child to this node, can be chained
    pub fn add(&self, node: T) -> Self {
        let id = self
            .graph
            .borrow_mut()
            .add_to(self.node, Node::new(node))
            .unwrap();

        GraphBuilder {
            graph: self.graph.clone(),
            node: id,
        }
    }

    pub fn id(&self) -> NodeId {
        self.node
    }

    pub fn looping(&self) -> Self {
        self.link_id(self.node);
        GraphBuilder {
            graph: self.graph.clone(),
            node: self.id(),
        }
    }

    pub fn link_id(&self, id: NodeId) -> Self {
        self.graph.borrow_mut().link(self.node, id).unwrap();
        GraphBuilder {
            graph: self.graph.clone(),
            node: self.id(),
        }
    }

    pub fn link(&self, node: &Self) -> Self {
        self.graph.borrow_mut().link(self.node, node.id()).unwrap();
        GraphBuilder {
            graph: self.graph.clone(),
            node: self.id(),
        }
    }
}
