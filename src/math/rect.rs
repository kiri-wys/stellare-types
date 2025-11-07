use std::marker::PhantomData;

use stellare_types_derive::{BcArithmetic, BcBitops, CwArithmetic, CwBitops};

use crate::math::{Integer, Unit, Vector, Vector2};

pub type Rect2u<U> = Rect2<u32, U>;
pub type Rect2i<U> = Rect2<i32, U>;
pub type Rect2f<U> = Rect2<f32, U>;
pub type Rect2d<U> = Rect2<f64, U>;

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Rect2<I, U = ()>
where
    I: Integer,
    U: Unit,
{
    min: Vector2<I, U>,
    max: Vector2<I, U>,
    _phantom: PhantomData<U>,
}
impl<I, U> Rect2<I, U>
where
    I: Integer,
    U: Unit,
{
    pub fn empty() -> Self {
        Self {
            min: Vector2::new(I::max_value(), I::max_value()),
            max: Vector2::new(I::min_value(), I::min_value()),
            _phantom: PhantomData,
        }
    }
    pub fn new(mut min: Vector2<I, U>, mut max: Vector2<I, U>) -> Self {
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
    pub fn from_size(origin: Vector2<I, U>, size: Vector2<I, U>) -> Self {
        let max = size.max(Vector2::zero()) + origin;
        Self {
            min: origin,
            max,
            _phantom: PhantomData,
        }
    }
    pub fn contains_point(&self, point: &Vector2<I, U>) -> bool {
        let x = self.min.x <= point.x && self.max.x >= point.x;
        let y = self.min.y <= point.y && self.max.y >= point.y;
        x && y
    }
    pub fn add_point(&mut self, point: &Vector2<I, U>) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
    }
    pub fn union(&mut self, other: &Rect2<I, U>) {
        self.add_point(&other.min);
        self.add_point(&other.max);
    }

    #[inline]
    pub fn min(&self) -> Vector2<I, U> {
        self.min
    }
    #[inline]
    pub fn max(&self) -> Vector2<I, U> {
        self.max
    }
}

impl<I, U> From<Rect2<I, U>> for [I; 4]
where
    I: Integer,
    U: Unit,
{
    fn from(value: Rect2<I, U>) -> Self {
        [value.min.x, value.min.y, value.max.x, value.max.y]
    }
}

#[derive(Debug, Default, Clone, Copy, CwArithmetic, CwBitops, BcArithmetic, BcBitops)]
pub struct CornerData<I>
where
    I: Integer,
{
    pub top_left: I,
    pub bottom_left: I,
    pub bottom_right: I,
    pub top_right: I,
}
impl<I> CornerData<I>
where
    I: Integer,
{
    pub fn new(top_left: I, bottom_left: I, bottom_right: I, top_right: I) -> Self {
        Self {
            top_left,
            bottom_left,
            bottom_right,
            top_right,
        }
    }
    pub fn splat(value: I) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn clamp_components(&mut self, min: I, max: I) {
        self.top_left = self.top_left.clamp(min, max);
        self.bottom_left = self.bottom_left.clamp(min, max);
        self.bottom_right = self.bottom_right.clamp(min, max);
        self.top_right = self.top_right.clamp(min, max);
    }
}

impl<I> From<CornerData<I>> for [I; 4]
where
    I: Integer,
{
    fn from(value: CornerData<I>) -> Self {
        [
            value.top_left,
            value.bottom_left,
            value.bottom_right,
            value.top_right,
        ]
    }
}
