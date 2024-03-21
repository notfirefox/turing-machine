use crate::turing::{DeltaResult, Move::Left, Move::Right, BLANK};

#[must_use]
pub const fn accept1(state: usize) -> bool {
    state == 3
}

#[must_use]
pub const fn delta1(state: usize, input: char) -> Option<DeltaResult> {
    let (state, output, r#move) = match (state, input) {
        (0, '0' | '1') => (0, input, Right),
        (0, BLANK) => (1, BLANK, Left),
        (1, '1') => (1, '0', Left),
        (1, '0' | BLANK) => (2, '1', Left),
        (2, '0' | '1') => (2, input, Left),
        (2, BLANK) => (3, BLANK, Right),
        (_, _) => return None,
    };
    Some(DeltaResult::new(state, output, r#move))
}
