use crate::{
    input::Event,
    pattern::playground::first::{
        atom::KeyboardAtomCommand,
        pattern::{Pattern, PatternErr},
    },
};
use colored::*;

// todo: test cycles (1/2)
#[test]
fn basic() {
    let root = KeyboardAtomCommand::any(0);

    let mut pattern = Pattern::new(root, 0);

    let root = pattern.build();
    root.add(KeyboardAtomCommand::down(1, 1));
    root.add(KeyboardAtomCommand::down(2, 2));
    root.add(KeyboardAtomCommand::down(3, 0))
        .add(KeyboardAtomCommand::up(3, 3));

    root.add(KeyboardAtomCommand::down(4, 0))
        .add(KeyboardAtomCommand::wait(100, 0))
        .add(KeyboardAtomCommand::up(4, 4));

    root.add(KeyboardAtomCommand::down(5, 5)).looping();

    dbg!(&pattern);

    #[rustfmt::skip]
    let test_data = [
        (Event::down(1, 0), Ok(1)),
        (Event::down(2, 0), Ok(2)),

        (Event::down(3, 0), Ok(0)),
        (Event::up(3, 0), Ok(3)),

        (Event::down(4, 0), Ok(0)),
        (Event::up(4, 100), Err(PatternErr::Dead)), // (released too early)

        (Event::down(4, 1000), Ok(0)), // the state is now reset so we start over
        (Event::up(4, 1101), Ok(4)),

        (Event::down(5, 0), Ok(5)),
        (Event::down(5, 0), Ok(5)),
        (Event::down(1, 0), Err(PatternErr::Dead)),

        (Event::down(6, 0), Err(PatternErr::Dead)),
        (Event::down(1, 0), Ok(1)), // double check that state is not corrupted
    ];

    for (i, (input, expected_result)) in test_data.iter().enumerate() {
        let result = pattern.apply(&input);
        assert_eq!(&result, expected_result);
        println!("case {i} {}\n", "passed".green());
    }
}
