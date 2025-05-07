use crate::{
    input::{Event, KeyboardInput},
    pattern::{self, atom::Atom},
};

use super::pattern::PatternState;

pub type KeyboardAtom = pattern::KeyboardAtom<u8, u32>;

#[derive(Debug)]
pub enum AtomResult {
    /// input matched this Atom
    Accepted,
    /// input does not match this Atom
    Rejected,
    /// like 'Accepted', but requires to check children of the node straight away
    Proceed,
}

impl Atom for KeyboardAtom {
    type Input = Event<KeyboardInput, u32>;
    type State = PatternState;
    type Output = AtomResult;

    fn handle(&self, input: &Self::Input, state: &Self::State) -> Self::Output {
        match self {
            KeyboardAtom::Any => AtomResult::Accepted,
            KeyboardAtom::Down(c) => {
                if *c == input.event.code && input.event.state == 1 {
                    return AtomResult::Accepted;
                }
                AtomResult::Rejected
            }
            KeyboardAtom::Up(c) => {
                if *c == input.event.code && input.event.state == 2 {
                    return AtomResult::Accepted;
                }
                AtomResult::Rejected
            }
            KeyboardAtom::Wait(t) => {
                if input.time - state.time > *t {
                    return AtomResult::Proceed;
                }
                AtomResult::Rejected
            }
        }
    }
}
