use crate::turing::{DeltaParam, DeltaResult, Move, BLANK};

pub fn accept(q: &i32) -> bool {
    q == &2
}

pub fn delta(param: &DeltaParam) -> DeltaResult {
    if param.state == 0 {
        if param.input == 'a' {
            return DeltaResult::new(0, 'b', Move::Right);
        } else if param.input == 'b' {
            return DeltaResult::new(0, 'a', Move::Right);
        } else if param.input == 'c' || param.input == 'd' {
            return DeltaResult::new(0, param.input, Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(1, BLANK, Move::Left);
        }
    } else if param.state == 1 {
        if param.input == 'a' || param.input == 'b' || param.input == 'c' || param.input == 'd' {
            return DeltaResult::new(1, param.input, Move::Left);
        } else if param.input == BLANK {
            return DeltaResult::new(2, BLANK, Move::Right);
        }
    }

    DeltaResult::new(-1, BLANK, Move::None)
}
