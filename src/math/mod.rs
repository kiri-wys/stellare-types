pub mod angles;
pub mod rect;
pub mod transform2;
pub mod vec2;

use core::{f32, f64};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub use crate::math::{
    angles::{Angle, Degrees, Radians},
    rect::{Rect2, Rect2d, Rect2f, Rect2i, Rect2u},
    transform2::Affine2,
    vec2::{Vector2, Vector2d, Vector2f, Vector2i, Vector2u},
};

pub trait Decimal: Clone + Copy + Scalar<Self> + Neg<Output = Self> {
    fn pi() -> Self;
    fn tau() -> Self;
    fn to_radians(self) -> Self;
    fn to_degrees(self) -> Self;

    fn sqrt(&self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn atan2(self, other: Self) -> Self;

    fn is_number(self) -> bool;
    fn can_div_safe(self) -> bool;
}
impl Decimal for f32 {
    fn pi() -> Self {
        f32::consts::PI
    }
    fn tau() -> Self {
        f32::consts::TAU
    }
    fn to_radians(self) -> Self {
        self.to_radians()
    }
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
    fn cos(self) -> Self {
        f32::cos(self)
    }
    fn sin(self) -> Self {
        f32::sin(self)
    }
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn is_number(self) -> bool {
        !(self.is_nan() || self.is_infinite())
    }
    fn can_div_safe(self) -> bool {
        !(self.is_number() || self <= 0.0)
    }
}
impl Decimal for f64 {
    fn pi() -> Self {
        f64::consts::PI
    }
    fn tau() -> Self {
        f64::consts::TAU
    }
    fn to_radians(self) -> Self {
        self.to_radians()
    }
    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
    fn cos(self) -> Self {
        f64::cos(self)
    }
    fn sin(self) -> Self {
        f64::sin(self)
    }
    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn is_number(self) -> bool {
        !(self.is_nan() || self.is_infinite())
    }
    fn can_div_safe(self) -> bool {
        !(self.is_number() || self <= 0.0)
    }
}

pub trait Scalar<D: Decimal>:
    Clone
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
{
    fn zero() -> Self;
    fn one() -> Self;

    fn to_precise(self) -> D;

    fn min(self, other: Self) -> Self {
        if self > other { other } else { self }
    }
    fn max(self, other: Self) -> Self {
        if self < other { other } else { self }
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl Scalar<f32> for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn to_precise(self) -> f32 {
        self
    }
}
impl Scalar<f64> for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn to_precise(self) -> f64 {
        self
    }
}

macro_rules! impl_scalar_for_ints {
    ($($t:ty),*) => {
        $(
            impl Scalar<f32> for $t {
                fn zero() -> Self {
                    0
                }
                fn one() -> Self {
                    1
                }
                fn to_precise(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}

impl_scalar_for_ints!(u8, i8, u16, i16, u32, i32);
macro_rules! impl_scalar_for_64 {
    ($($t:ty),*) => {
        $(
            impl Scalar<f64> for $t {
                fn zero() -> Self {
                    0
                }
                fn one() -> Self {
                    1
                }
                fn to_precise(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}
impl_scalar_for_64!(i64, u64);

pub trait Unit: Clone + Copy {}
impl Unit for () {}

macro_rules! define_spaces {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            impl Unit for $name {}
        )*
    };
}
define_spaces!(WorldSpace, ViewSpace, ClipSpace, TexelSpace, ScreenSpace);

pub trait Vector<D, S>
where
    D: Decimal,
    S: Scalar<D>,
{
    type Precise;
    type Normalized;

    fn zero() -> Self;
    fn one() -> Self;

    fn to_precise(self) -> Self::Precise;

    fn dot(self, other: Self) -> S;
    fn length_squared(self) -> S;
    fn length(self) -> D;
    fn normalize(self) -> Self::Normalized;
    fn distance_to(self, other: Self) -> D;
    fn distance_to_squared(self, other: Self) -> S;

    fn rotate<A: Angle<D>>(self, angle: A) -> Self::Precise;
    fn lerp(self, max: Self, alpha: D) -> Self::Precise;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn min_component(self) -> (usize, S);
    fn max_component(self) -> (usize, S);

    fn magnitude_squared(self) -> S
    where
        Self: Sized,
    {
        self.length_squared()
    }
    fn magnitude(self) -> D
    where
        Self: Sized,
    {
        self.length()
    }
}

#[repr(transparent)]
pub struct NormalizedVector2<D, U = ()>
where
    D: Decimal + Scalar<D>,
    U: Unit,
{
    data: Vector2<D, D, U>,
}
impl<D, U> NormalizedVector2<D, U>
where
    D: Decimal,
    U: Unit,
{
    pub fn vector(self) -> Vector2<D, D, U> {
        self.data
    }
}
