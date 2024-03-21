use crate::turing::{DeltaResult, Move, BLANK};

#[must_use]
pub const fn accept5(state: usize) -> bool {
    state == 5
}

#[must_use]
pub const fn delta5(state: usize, input: char) -> Option<DeltaResult> {
    if state == 0 {
        if input == 'a' {
            return Some(DeltaResult::new(1, 'A', Move::Right));
        }
    } else if state == 1 {
        if input == 'X' {
            return Some(DeltaResult::new(1, 'X', Move::Right));
        } else if input == 'a' {
            return Some(DeltaResult::new(2, 'X', Move::Right));
        } else if input == BLANK {
            return Some(DeltaResult::new(5, BLANK, Move::None));
        }
    } else if state == 2 {
        if input == 'X' {
            return Some(DeltaResult::new(2, 'X', Move::Right));
        } else if input == 'a' {
            return Some(DeltaResult::new(3, 'a', Move::Right));
        } else if input == BLANK {
            return Some(DeltaResult::new(4, BLANK, Move::Left));
        }
    } else if state == 3 {
        if input == 'a' {
            return Some(DeltaResult::new(2, 'X', Move::Right));
        } else if input == 'X' {
            return Some(DeltaResult::new(3, 'X', Move::Left));
        }
    } else if state == 4 {
        if input == 'X' || input == 'a' {
            return Some(DeltaResult::new(4, input, Move::Left));
        } else if input == 'A' {
            return Some(DeltaResult::new(1, 'A', Move::Right));
        }
    }

    None
}
