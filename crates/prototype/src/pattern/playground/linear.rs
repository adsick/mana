// this is a playground implementing and testing a simple linear pattern.
// linear means it contains no branches (and therefore it is nearly useless in practice)
// but it is needed for learning purposes

use std::collections::{vec_deque, VecDeque};

use crate::{input::Event, input::KeyboardInput};

use crate::pattern::atom::KeyboardAtom;

struct LinearPattern(Vec<KeyboardAtom<u8, u32>>);
type LinearPatternState = (usize, u32); // position and last timestamp

struct Events(VecDeque<Event<KeyboardInput, u32>>);

impl Events {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::iter::IntoIterator for Events {
    type Item = Event<KeyboardInput, u32>;

    type IntoIter = vec_deque::IntoIter<Event<KeyboardInput, u32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const N: usize> From<[(u8, u8, u32); N]> for Events {
    fn from(value: [(u8, u8, u32); N]) -> Self {
        Self(value.into_iter().map(|t| t.into()).collect())
    }
}

// impl Pattern<Events, LinearPatternState, (), bool> for LinearPattern {
// fn apply(&self, input: Events, state: &mut LinearPatternState, _ctx: &()) -> bool {
impl LinearPattern {
    fn apply(&self, mut input: Events, state: &mut LinearPatternState) -> bool {
        let (pos, last_timestamp) = state;

        while let Some(atom) = self.0.get(*pos) {
            if let Some(Event { event: e, time: t }) = input.0.front() {
                // println!("pos: {pos}\n--=--");
                // println!("event: {:?}\natom: {:?}\n", e, atom);

                match *atom {
                    KeyboardAtom::Any => {
                        let code = e.code;
                        if e.state == 1 {
                            print!("+Press(Any({code}))")
                        } else {
                            print!("+Release(Any({code}))")
                        }
                        *last_timestamp = *t;
                    }
                    KeyboardAtom::Down(code) => {
                        if e.code != code || e.state != 1 {
                            println!("\n!Press({code})");
                            return false;
                        }
                        print!("+Press({code})");

                        *last_timestamp = *t;
                        input.0.pop_front(); // consume event
                    }
                    KeyboardAtom::Up(code) => {
                        if e.code != code || e.state != 0 {
                            println!("\n!Release({code})");
                            return false;
                        }
                        print!("+Release({code})");

                        *last_timestamp = *t;
                        input.0.pop_front();
                    }
                    KeyboardAtom::Wait(duration) => {
                        if t - *last_timestamp < duration {
                            println!("\n!Wait({duration})");
                            return false;
                        }
                        print!("+Wait({duration})");

                        *last_timestamp = *t; // NS
                    }
                }
                *pos += 1;
                // println!("\n-=-");
            } else {
                println!("incomplete");
                return false;
            }
        }
        *pos = 0;
        println!();
        true
    }
}

#[test]
fn basic() {
    use KeyboardAtom::*;

    let patt = LinearPattern(vec![Wait(200), Down(5), Wait(100), Down(4), Up(5), Up(4)]);

    let mut state: LinearPatternState = (0, 0);
    let mut res;

    // incomplete input
    let events = [(5, 1, 201)].into();
    res = patt.apply(events, &mut state);
    println!("res = {res}, state: {:?}\n\n", state);
    assert_eq!(res, false);

    state = (0, 0);
    let events = [(5, 1, 100), (4, 1, 200), (5, 0, 400), (4, 0, 500)].into();
    res = patt.apply(events, &mut state);
    println!("res = {res}, state: {:?}\n\n", state);
    assert_eq!(res, false);

    state = (0, 0);
    let events = [(5, 1, 201), (4, 1, 300), (5, 0, 400), (4, 0, 500)].into();
    res = patt.apply(events, &mut state);
    println!("res = {res}, state: {:?}\n\n", state);
    assert_eq!(res, false);

    state = (1, 0); // note, we starting the pattern from step 1 (Press(5)) and it still is valid
    let events = [(5, 1, 100), (4, 1, 301), (5, 0, 400), (4, 0, 500)].into();
    res = patt.apply(events, &mut state);
    println!("res = {res}, state: {:?}\n\n", state);
    assert_eq!(res, true);
}
