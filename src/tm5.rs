use crate::turing::{DeltaParam, DeltaResult, Move, BLANK};

pub fn accept(q: &i32) -> bool {
    q == &5
}

pub fn delta(param: &DeltaParam) -> DeltaResult {
    if param.state == 0 {
        if param.input == 'a' {
            return DeltaResult::new(1, 'A', Move::Right);
        } 
    } else if param.state == 1 {
        if param.input == 'X' {
            return DeltaResult::new(1, 'X', Move::Right);
        } else if param.input == 'a' {
            return DeltaResult::new(2, 'X', Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(5, BLANK, Move::None);
        }
    } else if param.state == 2 {
        if param.input == 'X' {
            return DeltaResult::new(2, 'X', Move::Right);
        } else if param.input == 'a' {
            return DeltaResult::new(3, 'a', Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(4, BLANK, Move::Left);
        }
    } else if param.state == 3 {
        if param.input == 'a' {
            return DeltaResult::new(2, 'X', Move::Right);
        } else if param.input == 'X' {
            return DeltaResult::new(3, 'X', Move::Left);
        }
    } else if param.state == 4 {
        if param.input == 'X' || param.input == 'a' {
            return DeltaResult::new(4, param.input, Move::Left);
        } else if param.input == 'A' {
            return DeltaResult::new(1, 'A', Move::Right);
        }
    }

    DeltaResult::new(-1, BLANK, Move::None)
}
