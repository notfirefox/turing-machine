use crate::turing::{DeltaParam, DeltaResult, Move, BLANK};

pub fn accept(q: &i32) -> bool {
    q == &8
}

pub fn delta(param: &DeltaParam) -> DeltaResult {
    if param.state == 0 {
        if param.input == 'a' {
            return DeltaResult::new(0, 'A', Move::Right);
        } else if param.input == 'b' {
            return DeltaResult::new(0, 'B', Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(1, BLANK, Move::Left);
        }
    } else if param.state == 1 {
        if param.input == 'a' || param.input == 'b' || param.input == 'A' || param.input == 'B' {
            return DeltaResult::new(1, param.input, Move::Left);
        } else if param.input == BLANK {
            // state = 2 => found first A or B
            return DeltaResult::new(2, BLANK, Move::Right);
        }
    } else if param.state == 2 {
        if param.input == 'A' {
            // state = 3 => append a
            return DeltaResult::new(3, 'a', Move::Right);
        } else if param.input == 'B' {
            // state 4 => append b
            return DeltaResult::new(4, 'b', Move::Right);
        }
    } else if param.state == 3 {
        if param.input == 'a' || param.input == 'b' || param.input == 'A' || param.input == 'B' {
            return DeltaResult::new(param.state, param.input, Move::Right);
        } else if param.input == BLANK {
            // state = 5 => go left to next blank
            return DeltaResult::new(5, 'a', Move::Left);
        }
    } else if param.state == 4 {
        if param.input == 'a' || param.input == 'b' || param.input == 'A' || param.input == 'B' {
            return DeltaResult::new(param.state, param.input, Move::Right);
        } else if param.input == BLANK {
            // state = 5 => go left to next blank
            return DeltaResult::new(5, 'b', Move::Left);
        }
    } else if param.state == 5 {
        if param.input == 'a' || param.input == 'b' || param.input == 'A' || param.input == 'B' {
            return DeltaResult::new(5, param.input, Move::Left);
        } else if param.input == BLANK {
            // state = 6 => reached first character
            return DeltaResult::new(6, BLANK, Move::Right);
        }
    } else if param.state == 6 {
        if param.input == 'a' || param.input == 'b' {
            // still searching for A or B
            return DeltaResult::new(6, param.input, Move::Right);
        } else if param.input == 'A' || param.input == 'B' {
            // found A or B
            return DeltaResult::new(2, param.input, Move::None);
        } else if param.input == BLANK {
            // reached end without finding A or B
            return DeltaResult::new(7, BLANK, Move::Left);
        }
    } else if param.state == 7 {
        if param.input == 'a' || param.input == 'b' {
            // did not find leftmost BLANK yet
            return DeltaResult::new(7, param.input, Move::Left);
        } else if param.input == BLANK {
            // found beginning of word
            return DeltaResult::new(8, BLANK, Move::Right);
        }
    }

    DeltaResult::new(-1, BLANK, Move::None)
}
