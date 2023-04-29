use crate::pattern::atom::Atom;


// the usefulness of this is doubtable
// todo: naming
#[derive(Debug)]
pub enum PatternCombinator<P: Atom> {
    Pattern(P),
    Not(P),
    Any(Vec<P>),
    All(Vec<P>),
}

impl<P> Atom for PatternCombinator<P>
where
    P: Atom<Output = bool>,
{
    type Input = P::Input;

    type State = P::State;

    type Output = bool;

    fn handle(&self, input: &Self::Input, state: &Self::State) -> Self::Output {
        match self {
            PatternCombinator::Pattern(p) => p.handle(input, state),
            PatternCombinator::Not(p) => !p.handle(input, state),
            PatternCombinator::Any(ps) => ps.iter().map(|p| p.handle(input, state)).any(|p| p),
            PatternCombinator::All(ps) => ps.iter().map(|p| p.handle(input, state)).all(|p| p),
        }
    }
}
