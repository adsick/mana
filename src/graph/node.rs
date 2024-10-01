use std::slice::Iter;

pub type NodeId = usize;

#[derive(Debug, PartialEq)]
pub struct Node<V, E> {
    value: V,

    // decide what is 'next' and what is 'child'
    /// this nodes will be updated after this node completes
    /// "what to update next"
    next: Vec<(E, NodeId)>, // todo: smallvec(?),

    /// these are preconditions for this node (can be empty)
    /// "what conditions need to be met before this node can be completed"
    prev: Vec<NodeId>,
}

impl<V, E> Node<V, E> {
    pub fn new(value: V) -> Self {
        Self {
            value,
            next: vec![],
            prev: vec![],
        }
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn edges(&self) -> &[(E, NodeId)] {
        &self.next
    }

    pub fn edges_mut(&mut self) -> &mut [(E, NodeId)] {
        &mut self.next
    }

    // pub fn children(&self) -> Iter<NodeId> {
    //     self.next.iter().map(|e|e.1)
    // }

    // would be nice to overload it for E = (), but Rust doesn't allow to do so
    // pub fn add_edge(&mut self, id: NodeId) -> EdgeBuilder<'_, V, E> {
    //     EdgeBuilder::new(self)
    // }

    pub fn add_edge(&mut self, id: NodeId, val: E) {
        self.next.push((val, id))
    }

    /// returns true if the node has no outgoing edges
    pub fn leaf(&self) -> bool {
        self.next.is_empty()
    }
}

impl<V, E: Default> Node<V, E> {
    pub fn add_default_edge(&mut self, id: NodeId) {
        self.add_edge(id, E::default())
    }
}

impl<V: Default, E> Default for Node<V, E> {
    fn default() -> Self {
        Node {
            value: Default::default(),
            next: vec![],
            prev: vec![],
        }
    }
}

// // not sure about below stuff, but maybe we will end up with this more domain specific structure

// #[derive(Debug, Default)]
// pub struct PatternNode<S, P, A> {
//     /// state of the node (usually represents the completeness of it)
//     state: S,

//     /// specific pattern (predicate) that decides whether to activate this node or not
//     patt: P,

//     /// a useful value contained in the node
//     item: A,
// }

// impl<S, P, A> PatternNode<S, P, A> {
//     pub fn new(state: S, patt: P, item: A) -> Node<Self> {
//         Node::new(Self { state, patt, item })
//     }

//     pub fn state(&self) -> &S {
//         &self.state
//     }

//     pub fn predicate(&self) -> &P {
//         &self.patt
//     }

//     pub fn attachment(&self) -> &A {
//         &self.item
//     }
// }
