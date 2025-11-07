use crate::math::{Integer, Unit, Vector2};

pub struct Line2<I, U>(Vector2<I, U>, Vector2<I, U>)
where
    I: Integer,
    U: Unit;
