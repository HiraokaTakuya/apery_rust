use crate::position::*;
use crate::search::*;
use crate::types::*;

pub fn evaluate(pos: &mut Position, _stack: &mut [Stack]) -> Value {
    pos.material()
}

pub fn evaluate_at_root(pos: &Position, _stack: &mut [Stack]) -> Value {
    pos.material()
}
