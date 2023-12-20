use crate::turing::{DeltaParam, DeltaResult, Move, BLANK};

pub fn accept(q: &i32) -> bool {
    q == &4
}

pub fn delta(param: &DeltaParam) -> DeltaResult {
    if param.state == 0 {
        if param.input == 'a' {
            return DeltaResult::new(1, 'A', Move::Right);
        } else if param.input == 'b' {
            return DeltaResult::new(2, 'B', Move::Right);
        }
    } else if param.state == 1 {
        if param.input == 'a' || param.input == 'b' {
            return DeltaResult::new(1, param.input, Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(3, 'a', Move::Left);
        }
    } else if param.state == 2 {
        if param.input == 'a' || param.input == 'b' {
            return DeltaResult::new(2, param.input, Move::Right);
        } else if param.input == BLANK {
            return DeltaResult::new(3, 'b', Move::Left);
        }
    } else if param.state == 3 {
        if param.input == BLANK {
            return DeltaResult::new(4, BLANK, Move::Right);
        } else if param.input == 'A' {
            return DeltaResult::new(0, 'a', Move::Right);
        } else if param.input == 'B' {
            return DeltaResult::new(0, 'b', Move::Right);
        }
    }

    DeltaResult::new(-1, BLANK, Move::None)
}
