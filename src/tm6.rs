use crate::turing::{DeltaResult, Move, BLANK};

#[must_use]
pub const fn accept6(state: usize) -> bool {
    state == 4
}

#[must_use]
pub const fn delta6(state: usize, input: char) -> Option<DeltaResult> {
    if state == 0 {
        if input == 'a' {
            return Some(DeltaResult::new(1, 'A', Move::Right));
        } else if input == 'b' {
            return Some(DeltaResult::new(2, 'B', Move::Right));
        }
    } else if state == 1 {
        if input == 'a' || input == 'b' {
            return Some(DeltaResult::new(1, input, Move::Right));
        } else if input == BLANK {
            return Some(DeltaResult::new(3, 'a', Move::Left));
        }
    } else if state == 2 {
        if input == 'a' || input == 'b' {
            return Some(DeltaResult::new(2, input, Move::Right));
        } else if input == BLANK {
            return Some(DeltaResult::new(3, 'b', Move::Left));
        }
    } else if state == 3 {
        if input == BLANK {
            return Some(DeltaResult::new(4, BLANK, Move::Right));
        } else if input == 'A' {
            return Some(DeltaResult::new(0, 'a', Move::Right));
        } else if input == 'B' {
            return Some(DeltaResult::new(0, 'b', Move::Right));
        }
    }

    None
}
