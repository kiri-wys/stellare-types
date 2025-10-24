use crate::math::{Scalar, Unit, Vector2};

pub struct Line2<S, U>(Vector2<S, U>, Vector2<S, U>)
where
    S: Scalar,
    U: Unit;
