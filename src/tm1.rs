use crate::turing::{DeltaParam, DeltaResult, Move, BLANK};

pub fn accept_1(q: &i32) -> bool {
    q == &3
}

pub fn delta_1(param: &DeltaParam) -> DeltaResult {
    if param.state == 0 {
        if param.input == '0' || param.input == '1' {
            return DeltaResult::new(0, param.input, Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(1, BLANK, Move::Left);
        }
    } else if param.state == 1 {
        if param.input == '1' {
            return DeltaResult::new(1, '0', Move::Left);
        } else if param.input == '0' || param.input == BLANK {
            return DeltaResult::new(2, '1', Move::Left);
        }
    } else if param.state == 2 {
        if param.input == '0' || param.input == '1' {
            return DeltaResult::new(2, param.input, Move::Left);
        } else if param.input == BLANK {
            return DeltaResult::new(3, BLANK, Move::Right);
        }
    }

    DeltaResult::new(-1, BLANK, Move::None)
}
