use std::{
    marker::PhantomData,
    ops::{Div, Mul},
};

use crate::math::{Decimal, Scalar, Unit, Vector, Vector2};

pub type Rect2u<U> = Rect2<f32, u32, U>;
pub type Rect2i<U> = Rect2<f32, i32, U>;
pub type Rect2f<U> = Rect2<f32, f32, U>;
pub type Rect2d<U> = Rect2<f64, i64, U>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect2<D, S, U = ()>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    min: Vector2<D, S, U>,
    max: Vector2<D, S, U>,
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

    #[inline]
    pub fn min(&self) -> Vector2<D, S, U> {
        self.min
    }
    #[inline]
    pub fn max(&self) -> Vector2<D, S, U> {
        self.max
    }
}

impl<D, S, U> From<Rect2<D, S, U>> for [S; 4]
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    fn from(value: Rect2<D, S, U>) -> Self {
        [value.min.x, value.min.y, value.max.x, value.max.y]
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CornerData<D, S>
where
    D: Decimal,
    S: Scalar<D>,
{
    pub top_left: S,
    pub bottom_left: S,
    pub bottom_right: S,
    pub top_right: S,
    _phantom_data: PhantomData<D>,
}
impl<D, S> CornerData<D, S>
where
    D: Decimal,
    S: Scalar<D>,
{
    pub fn new(top_left: S, bottom_left: S, bottom_right: S, top_right: S) -> Self {
        Self {
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            _phantom_data: PhantomData,
        }
    }
    pub fn splat(value: S) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn clamp_components(&mut self, min: S, max: S) {
        self.top_left = self.top_left.clamp(min, max);
        self.bottom_left = self.bottom_left.clamp(min, max);
        self.bottom_right = self.bottom_right.clamp(min, max);
        self.top_right = self.top_right.clamp(min, max);
    }
}
impl<D, S> Mul<S> for CornerData<D, S>
where
    D: Decimal,
    S: Scalar<D>,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self::Output {
            top_left: self.top_left * rhs,
            bottom_left: self.bottom_left * rhs,
            bottom_right: self.bottom_right * rhs,
            top_right: self.top_right * rhs,
            _phantom_data: PhantomData,
        }
    }
}

impl<D, S> Div<S> for CornerData<D, S>
where
    D: Decimal,
    S: Scalar<D>,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self::Output {
            top_left: self.top_left / rhs,
            bottom_left: self.bottom_left / rhs,
            bottom_right: self.bottom_right / rhs,
            top_right: self.top_right / rhs,
            _phantom_data: PhantomData,
        }
    }
}

impl<D, S> From<CornerData<D, S>> for [S; 4]
where
    D: Decimal,
    S: Scalar<D>,
{
    fn from(value: CornerData<D, S>) -> Self {
        [
            value.top_left,
            value.bottom_left,
            value.bottom_right,
            value.top_right,
        ]
    }
}
