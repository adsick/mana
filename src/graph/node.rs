use std::slice::Iter;

pub type NodeId = usize;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: T,

    // todo

    // decide what is 'next' and what is 'child'
    /// this nodes will be updated after this node completes
    /// "what to update next"
    next: Vec<NodeId>, // todo: smallvec(?),

    /// these are preconditions for this node (can be empty)
    /// "what conditions need to be met before this node can be completed"
    prev: Vec<NodeId>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: vec![],
            prev: vec![],
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn children(&self) -> Iter<NodeId> {
        self.next.iter()
    }

    pub fn add_child(&mut self, id: NodeId) {
        if self.next.contains(&id) {
            return;
        }
        self.next.push(id)
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node {
            value: Default::default(),
            next: vec![],
            prev: vec![],
        }
    }
}

// not sure about below stuff, but maybe we will end up with this more domain specific structure

#[derive(Debug, Default)]
pub struct PatternNode<S, P, A> {
    /// state of the node (usually represents the completeness of it)
    state: S,

    /// specific pattern (predicate) that decides whether to activate this node or not
    patt: P,

    /// a useful value contained in the node
    item: A,
}

impl<S, P, A> PatternNode<S, P, A> {
    pub fn new(state: S, patt: P, item: A) -> Node<Self> {
        Node::new(Self { state, patt, item })
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn predicate(&self) -> &P {
        &self.patt
    }

    pub fn attachment(&self) -> &A {
        &self.item
    }
}
