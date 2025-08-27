use std::marker::PhantomData;

use crate::math::{Decimal, Scalar, Unit, Vector, Vector2};

pub struct Rect2<D, S, U = ()>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    pub min: Vector2<D, S, U>,
    pub max: Vector2<D, S, U>,
    _phantom: PhantomData<U>,
}
impl<D, S, U> Rect2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    pub fn new(mut min: Vector2<D, S, U>, mut max: Vector2<D, S, U>) -> Self {
        if min.x > max.x {
            std::mem::swap(&mut min.x, &mut max.x);
        }
        if min.y > max.y {
            std::mem::swap(&mut min.y, &mut max.y);
        }
        Self {
            min,
            max,
            _phantom: PhantomData,
        }
    }
    pub fn from_size(origin: Vector2<D, S, U>, size: Vector2<D, S, U>) -> Self {
        let max = size.max(Vector2::zero()) + origin;
        Self {
            min: origin,
            max,
            _phantom: PhantomData,
        }
    }
}
