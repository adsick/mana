use crate::{
    input::Event,
    pattern::playground::first::{
        atom::KeyboardAtom,
        pattern::Pattern,
    },
};
use colored::*;

// todo: test cycles (1/2)
#[test]
fn basic() {
    let mut pattern = Pattern::new(0, 0);

    let root = pattern.build();
    root.add(KeyboardAtom::Down(1), 1);
    root.add(KeyboardAtom::Down(2), 2);
    root.add(KeyboardAtom::Down(3), 0)
        .add(KeyboardAtom::Up(3), 3);

    root.add(KeyboardAtom::Down(4), 0)
        .add(KeyboardAtom::Wait(100), 0)
        .add(KeyboardAtom::Up(4), 4);

    root.add(KeyboardAtom::Down(5), 5)
        .looping(KeyboardAtom::Down(5));

    dbg!(&pattern);

    #[rustfmt::skip]
    let test_data = [
        (Event::down(1, 0), Some(1)),
        (Event::down(2, 0), Some(2)),

        (Event::down(3, 0), Some(0)),
        (Event::up(3, 0), Some(3)),

        (Event::down(4, 0), Some(0)),
        (Event::up(4, 100), None), // (released too early)

        (Event::down(4, 1000), Some(0)), // the state is now reset so we start over
        (Event::up(4, 1101), Some(4)),

        (Event::down(5, 0), Some(5)),
        (Event::down(5, 0), Some(5)),
        (Event::down(1, 0), None),

        (Event::down(6, 0), None),
        (Event::down(1, 0), Some(1)), // double check that state is not corrupted
    ];

    for (i, (input, expected_result)) in test_data.iter().enumerate() {
        let i = i + 1;
        println!("{} case {i}", "testing".yellow());
        let result = pattern.apply(&input);
        assert_eq!(&result, expected_result);
        println!("case {i} {}\n", "passed".green());
    }
}
