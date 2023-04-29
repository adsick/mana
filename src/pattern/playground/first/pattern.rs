use std::collections::VecDeque;

use crate::{
    graph::{Graph, GraphBuilder, NodeId},
    input::Event,
    input::KeyboardInput,
    pattern::playground::first::atom::AtomResult,
};

use crate::pattern::atom::{Atom, KeyboardAtom};

use super::atom::KeyboardAtomCommand;

#[derive(Debug)]
pub struct Pattern<T, S> {
    graph: Graph<T>,
    state: S,
}

#[derive(Debug)]
pub struct PatternState {
    active: NodeId,
    pub time: u32, // there might be more different "times", like "root_time" for example (timestamp of the root activation)
}

impl Pattern<KeyboardAtomCommand, PatternState> {
    pub fn new(root: KeyboardAtomCommand, time: u32) -> Self {
        Self {
            graph: Graph::new(root),
            state: PatternState { active: 0, time },
        }
    }

    pub fn build(&mut self) -> GraphBuilder<'_, KeyboardAtomCommand> {
        self.graph.build()
    }

    // todo: consider proper enum for return type (it may contain info about the completeness of the pattern)
    /// call this method anytime you want to get new commands
    pub fn apply(&mut self, input: &Event<KeyboardInput, u32>) -> Result<u8, PatternErr> {
        // this is the node id whose children we match with our `input`
        let mut current = self.state.active;

        let mut queue = VecDeque::new(); // I believe we could use a simple vec here, but that may be related to 'priority' stuff

        queue.push_front(current);

        while let Some(current) = queue.pop_back() {
            for (&id, node) in self.graph.get_children(current) {
                let atom = node.value();

                let res = atom.handle(input, &self.state);

                println!("testing {:?} against {:?}\nresult: {:?}", input, atom, res);

                match res {
                    AtomResult::Accepted => {
                        self.state.active = id; // set current active node to this accepted child
                        self.state.time = input.time;

                        if self.graph.is_leaf_node(id) {
                            // found a node without children
                            // perform a self reset
                            self.state.active = 0;
                        }

                        return Ok(atom.command); // this will maybe replaced with a break
                    }
                    AtomResult::Rejected => continue,
                    AtomResult::Proceed => {
                        // maybe push this node's command to a vec and return it in the end?
                        queue.push_front(id) // check out that node
                    }
                }
            }
        }

        // reset on bad input
        self.state.active = 0;

        // kinda bad
        Err(PatternErr::Dead)
    }
}

#[derive(Debug, PartialEq)]
pub enum PatternErr {
    /// input not handled (unexpected input)
    Dead,
    /// input is incomplete
    Incomplete,
}
