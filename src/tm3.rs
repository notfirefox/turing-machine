use crate::turing::{DeltaResult, Move::Left, Move::Right, BLANK};

#[must_use]
pub const fn accept3(state: usize) -> bool {
    state == 2
}

#[must_use]
pub const fn delta3(state: usize, input: char) -> Option<DeltaResult> {
    let (state, output, r#move) = match (state, input) {
        (0, 'a') => (0, 'b', Right),
        (0, 'b') => (0, 'a', Right),
        (0, 'c') => (0, 'c', Right),
        (0, 'd') => (0, 'd', Right),
        (0, BLANK) => (1, BLANK, Left),
        (1, 'a') => (1, 'a', Left),
        (1, 'b') => (1, 'b', Left),
        (1, 'c') => (1, 'c', Left),
        (1, 'd') => (1, 'd', Left),
        (1, BLANK) => (2, BLANK, Right),
        (_, _) => return None,
    };
    Some(DeltaResult::new(state, output, r#move))
}
