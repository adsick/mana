use crate::{
    input::{Event, KeyboardInput},
    pattern::{atom::Atom, KeyboardAtom},
};

use super::pattern::PatternState;

#[derive(Debug, Default)]
pub struct KeyboardAtomCommand {
    pub atom: KeyboardAtom<u8, u32>,
    pub command: u8,
}

impl KeyboardAtomCommand {
    pub fn new(atom: KeyboardAtom<u8, u32>, command: u8) -> Self {
        Self { atom, command }
    }

    pub fn any(command: u8) -> Self {
        Self::new(KeyboardAtom::Any, command)
    }

    pub fn down(code: u8, command: u8) -> Self {
        Self::new(KeyboardAtom::Down(code), command)
    }

    pub fn up(code: u8, command: u8) -> Self {
        Self::new(KeyboardAtom::Up(code), command)
    }

    pub fn wait(time: u32, command: u8) -> Self {
        Self::new(KeyboardAtom::Wait(time), command)
    }
}

#[derive(Debug)]
pub enum AtomResult {
    /// input matched this Atom
    Accepted,
    /// input does not match this Atom
    Rejected,
    /// like 'Accepted', but requires to check children of the node straight away
    Proceed,
}

impl Atom for KeyboardAtomCommand {
    type Input = Event<KeyboardInput, u32>;
    type State = PatternState;
    type Output = AtomResult;

    fn handle(&self, input: &Self::Input, state: &Self::State) -> Self::Output {
        match self.atom {
            KeyboardAtom::Any => AtomResult::Accepted,
            KeyboardAtom::Down(c) => {
                if c == input.event.code && input.event.state == 1 {
                    return AtomResult::Accepted;
                }
                AtomResult::Rejected
            }
            KeyboardAtom::Up(c) => {
                if c == input.event.code && input.event.state == 2 {
                    return AtomResult::Accepted;
                }
                AtomResult::Rejected
            }
            KeyboardAtom::Wait(t) => {
                if input.time - state.time > t {
                    return AtomResult::Proceed;
                }
                AtomResult::Rejected
            }
        }
    }
}
