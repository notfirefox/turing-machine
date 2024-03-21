use crate::turing::{DeltaResult, Move, BLANK};

#[must_use]
pub const fn accept4(state: usize) -> bool {
    state == 8
}

#[must_use]
pub const fn delta4(state: usize, input: char) -> Option<DeltaResult> {
    if state == 0 {
        if input == 'a' {
            return Some(DeltaResult::new(0, 'A', Move::Right));
        } else if input == 'b' {
            return Some(DeltaResult::new(0, 'B', Move::Right));
        } else if input == BLANK {
            return Some(DeltaResult::new(1, BLANK, Move::Left));
        }
    } else if state == 1 {
        if input == 'a' || input == 'b' || input == 'A' || input == 'B' {
            return Some(DeltaResult::new(1, input, Move::Left));
        } else if input == BLANK {
            // state = 2 => found first A or B
            return Some(DeltaResult::new(2, BLANK, Move::Right));
        }
    } else if state == 2 {
        if input == 'A' {
            // state = 3 => append a
            return Some(DeltaResult::new(3, 'a', Move::Right));
        } else if input == 'B' {
            // state 4 => append b
            return Some(DeltaResult::new(4, 'b', Move::Right));
        }
    } else if state == 3 {
        if input == 'a' || input == 'b' || input == 'A' || input == 'B' {
            return Some(DeltaResult::new(state, input, Move::Right));
        } else if input == BLANK {
            // state = 5 => go left to next blank
            return Some(DeltaResult::new(5, 'a', Move::Left));
        }
    } else if state == 4 {
        if input == 'a' || input == 'b' || input == 'A' || input == 'B' {
            return Some(DeltaResult::new(state, input, Move::Right));
        } else if input == BLANK {
            // state = 5 => go left to next blank
            return Some(DeltaResult::new(5, 'b', Move::Left));
        }
    } else if state == 5 {
        if input == 'a' || input == 'b' || input == 'A' || input == 'B' {
            return Some(DeltaResult::new(5, input, Move::Left));
        } else if input == BLANK {
            // state = 6 => reached first character
            return Some(DeltaResult::new(6, BLANK, Move::Right));
        }
    } else if state == 6 {
        if input == 'a' || input == 'b' {
            // still searching for A or B
            return Some(DeltaResult::new(6, input, Move::Right));
        } else if input == 'A' || input == 'B' {
            // found A or B
            return Some(DeltaResult::new(2, input, Move::None));
        } else if input == BLANK {
            // reached end without finding A or B
            return Some(DeltaResult::new(7, BLANK, Move::Left));
        }
    } else if state == 7 {
        if input == 'a' || input == 'b' {
            // did not find leftmost BLANK yet
            return Some(DeltaResult::new(7, input, Move::Left));
        } else if input == BLANK {
            // found beginning of word
            return Some(DeltaResult::new(8, BLANK, Move::Right));
        }
    }

    None
}
