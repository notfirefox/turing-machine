use crate::turing::{DeltaResult, Move, BLANK};

#[must_use]
pub const fn accept3(state: usize) -> bool {
    state == 2
}

#[must_use]
pub const fn delta3(state: usize, input: char) -> Option<DeltaResult> {
    if state == 0 {
        if input == 'a' {
            return Some(DeltaResult::new(0, 'b', Move::Right));
        } else if input == 'b' {
            return Some(DeltaResult::new(0, 'a', Move::Right));
        } else if input == 'c' || input == 'd' {
            return Some(DeltaResult::new(0, input, Move::Right));
        } else if input == BLANK {
            return Some(DeltaResult::new(1, BLANK, Move::Left));
        }
    } else if state == 1 {
        if input == 'a' || input == 'b' || input == 'c' || input == 'd' {
            return Some(DeltaResult::new(1, input, Move::Left));
        } else if input == BLANK {
            return Some(DeltaResult::new(2, BLANK, Move::Right));
        }
    }

    None
}
