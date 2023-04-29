pub trait Atom {
    type Input;
    type State;
    type Output;
    fn handle(&self, input: &Self::Input, state: &Self::State) -> Self::Output; // mut state?
}

// observation: there is logic that can be applied to any automaton (Any, Wait)
// and there is logic, that is specific to keyboard domain: Down(code) and Up(code)

#[derive(Debug, Default)]
pub enum KeyboardAtom<K, T> {
    #[default]
    Any,
    Down(K),
    Up(K),
    Wait(T),
}