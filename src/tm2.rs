use crate::turing::{DeltaResult, Move::Left, Move::Right, BLANK};

#[must_use]
pub const fn accept2(state: usize) -> bool {
    state == 4
}

#[must_use]
pub const fn delta2(state: usize, input: char) -> Option<DeltaResult> {
    let (state, output, r#move) = match (state, input) {
        (0, '0') => (1, '1', Right),
        (1, '1') => (2, '0', Left),
        (2, '1') => (0, '1', Right),
        (1, BLANK) => (3, BLANK, Left),
        (3, '1') => (3, '1', Left),
        (3, BLANK) => (4, BLANK, Right),
        (_, _) => return None,
    };
    Some(DeltaResult::new(state, output, r#move))
}
