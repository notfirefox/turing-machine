use crate::turing::{DeltaParam, DeltaResult, Move, BLANK};

pub fn accept(q: &i32) -> bool {
    q == &3
}

pub fn delta(param: &DeltaParam) -> DeltaResult {
    if param.state == 0 {
        if param.input == '0' {
            return DeltaResult::new(1, '1', Move::Right);
        }
    } else if param.state == 1 {
        if param.input == '1' {
            return DeltaResult::new(2, '0', Move::Left);
        } else if param.input == BLANK {
            return DeltaResult::new(3, BLANK, Move::Right);
        }
    } else if param.state == 2 {
        if param.input == '1' {
            return DeltaResult::new(0, '1', Move::Right);
        }
    }

    DeltaResult::new(-1, BLANK, Move::None)
}
