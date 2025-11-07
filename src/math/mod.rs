pub mod angles;
pub mod bezier;
pub mod line2;
pub mod rect;
pub mod transform2;
pub mod vec2;

use core::{f32, f64};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

use crate::define_spaces;
pub use crate::math::{
    angles::{Angle, Degrees, Radians},
    rect::{Rect2, Rect2d, Rect2f, Rect2i, Rect2u},
    transform2::Affine2,
    vec2::{Vector2, Vector2d, Vector2f, Vector2i, Vector2u},
};

pub trait Decimal: Clone + Copy + Integer<Decimal = Self> + Neg<Output = Self> {
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
        self.is_number() && self != 0.0
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
        self.is_number() && self != 0.0
    }
}

pub trait Integer:
    Clone
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + PartialOrd
    + Display
{
    type Decimal: Decimal;

    fn zero() -> Self;
    fn one() -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn splat<V: Vector<Self>>(self) -> V;

    fn to_precise(self) -> Self::Decimal;

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

impl Integer for f32 {
    type Decimal = Self;
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn min_value() -> Self {
        Self::MIN
    }
    fn max_value() -> Self {
        Self::MAX
    }
    fn splat<V: Vector<Self>>(self) -> V {
        V::splat(self)
    }
    fn to_precise(self) -> f32 {
        self
    }
}
impl Integer for f64 {
    type Decimal = Self;
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn min_value() -> Self {
        Self::MIN
    }
    fn max_value() -> Self {
        Self::MAX
    }
    fn splat<V: Vector<Self>>(self) -> V {
        V::splat(self)
    }
    fn to_precise(self) -> f64 {
        self
    }
}

macro_rules! impl_integer_for_ints {
    ($($t:ty),*) => {
        $(
            impl Integer for $t {
                type Decimal = f32;
                fn zero() -> Self {
                    0
                }
                fn one() -> Self {
                    1
                }
                fn min_value() -> Self {
                    Self::MIN
                }
                fn max_value() -> Self {
                    Self::MAX
                }
                fn splat<V: Vector<Self>>(self) -> V {
                    V::splat(self)
                }
                fn to_precise(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}

impl_integer_for_ints!(u8, i8, u16, i16, u32, i32);
macro_rules! impl_integer_for_64 {
    ($($t:ty),*) => {
        $(
            impl Integer for $t {
                type Decimal = f64;
                fn zero() -> Self {
                    0
                }
                fn one() -> Self {
                    1
                }
                fn min_value() -> Self {
                    Self::MIN
                }
                fn max_value() -> Self {
                    Self::MAX
                }
                fn splat<V: Vector<Self>>(self) -> V {
                    V::splat(self)
                }
                fn to_precise(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}
impl_integer_for_64!(i64, u64);

pub trait Unit: Clone + Copy {}
impl Unit for () {}

define_spaces!(
    Unit,
    WorldSpace,
    ViewSpace,
    ClipSpace,
    TexelSpace,
    ScreenSpace
);

pub trait Vector<I>
where
    I: Integer,
{
    type Precise;
    type Normalized;

    fn zero() -> Self;
    fn one() -> Self;
    fn splat(val: I) -> Self;

    fn to_precise(self) -> Self::Precise;

    fn cross(self, other: Self) -> I;
    fn dot(self, other: Self) -> I;
    fn length_squared(self) -> I;
    fn length(self) -> I::Decimal;
    fn normalize(self) -> Self::Normalized;
    fn distance_to(self, other: Self) -> I::Decimal;
    fn distance_to_squared(self, other: Self) -> I;
    fn angle(self) -> Radians<I::Decimal>;

    fn rotate<A: Angle<I::Decimal>>(self, angle: A) -> Self::Precise;
    fn lerp(self, max: Self, alpha: I::Decimal) -> Self::Precise;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn min_component(self) -> (usize, I);
    fn max_component(self) -> (usize, I);

    fn magnitude_squared(self) -> I
    where
        Self: Sized,
    {
        self.length_squared()
    }
    fn magnitude(self) -> I::Decimal
    where
        Self: Sized,
    {
        self.length()
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct NormalizedVector2<D, U = ()>
where
    D: Decimal,
    U: Unit,
{
    data: Vector2<D, U>,
}
impl<D, U> NormalizedVector2<D, U>
where
    D: Decimal,
    U: Unit,
{
    pub fn vector(self) -> Vector2<D, U> {
        self.data
    }
}
