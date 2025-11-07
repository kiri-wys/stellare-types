use std::{marker::PhantomData, ops::Neg};

use stellare_types_derive::{BcArithmetic, BcBitops, CwArithmetic, CwBitops};

use crate::math::{Angle, Decimal, Integer, NormalizedVector2, Radians, Unit, Vector};

pub type Vector2u<U> = Vector2<u32, U>;
pub type Vector2i<U> = Vector2<i32, U>;
pub type Vector2f<U> = Vector2<f32, U>;
pub type Vector2d<U> = Vector2<f64, U>;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, CwArithmetic, CwBitops, BcArithmetic, BcBitops,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Vector2<I, U = ()>
where
    I: Integer,
    U: Unit,
{
    pub x: I,
    pub y: I,
    #[op_override("PhantomData")]
    #[serde(skip)]
    _phantom: PhantomData<U>,
}
impl<I, U> Vector2<I, U>
where
    I: Integer,
    U: Unit,
{
    pub fn new(x: I, y: I) -> Vector2<I, U> {
        Vector2 {
            x,
            y,
            _phantom: PhantomData,
        }
    }

    pub fn inner_into<N: Integer + From<I>>(self) -> Vector2<N, U> {
        Vector2::new(N::from(self.x), N::from(self.y))
    }
}

impl<I, U> Vector<I> for Vector2<I, U>
where
    I: Integer,
    U: Unit,
{
    type Precise = Vector2<I::Decimal, U>;
    type Normalized = Option<NormalizedVector2<I::Decimal, U>>;

    fn zero() -> Self {
        Self {
            x: I::zero(),
            y: I::zero(),
            _phantom: PhantomData,
        }
    }
    fn one() -> Self {
        Self {
            x: I::one(),
            y: I::one(),
            _phantom: PhantomData,
        }
    }
    fn splat(val: I) -> Self {
        Self::new(val, val)
    }

    fn to_precise(self) -> Self::Precise {
        Vector2 {
            x: self.x.to_precise(),
            y: self.y.to_precise(),
            _phantom: PhantomData,
        }
    }

    fn cross(self, other: Self) -> I {
        self.x * other.y - self.y * other.x
    }
    fn dot(self, other: Self) -> I {
        self.x * other.x + self.y * other.y
    }
    fn length_squared(self) -> I {
        self.x * self.x + self.y * self.y
    }
    fn length(self) -> I::Decimal {
        self.length_squared().to_precise().sqrt()
    }
    fn normalize(self) -> Self::Normalized {
        let l = self.length();
        let p = self.to_precise();
        let is_valid = p.x.is_number() && p.y.is_number();
        if !l.can_div_safe() || !is_valid {
            return None;
        }
        Some(NormalizedVector2 { data: (p / l) })
    }
    fn distance_to(self, other: Self) -> I::Decimal {
        (self - other).length()
    }
    fn distance_to_squared(self, other: Self) -> I {
        (self - other).length_squared()
    }
    fn angle(self) -> Radians<I::Decimal> {
        let p = self.to_precise();
        Radians(p.y.atan2(p.x))
    }

    fn rotate<A>(self, angle: A) -> Self::Precise
    where
        A: Angle<I::Decimal>,
    {
        let rads = angle.radians();
        let p = self.to_precise();
        Vector2 {
            x: p.x * rads.cos() - p.y * rads.sin(),
            y: p.x * rads.sin() + p.y * rads.cos(),
            _phantom: PhantomData,
        }
    }
    fn lerp(self, max: Self, alpha: I::Decimal) -> Self::Precise {
        let p_min = self.to_precise();
        let p_max = max.to_precise();
        Vector2 {
            x: p_min.x * (I::Decimal::one() - alpha) + p_max.x * alpha,
            y: p_min.y * (I::Decimal::one() - alpha) + p_max.y * alpha,
            _phantom: PhantomData,
        }
    }
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            _phantom: PhantomData,
        }
    }
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            _phantom: PhantomData,
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            _phantom: PhantomData,
        }
    }
    fn min_component(self) -> (usize, I) {
        if self.x <= self.y {
            (0, self.x)
        } else {
            (1, self.y)
        }
    }
    fn max_component(self) -> (usize, I) {
        if self.x >= self.y {
            (0, self.x)
        } else {
            (1, self.y)
        }
    }
}

macro_rules! impl_neg_for_signed {
    ($($t:ty),*) => {
        $(
            impl<U> Neg for Vector2<$t, U>
            where
                U: Unit,
            {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self {
                        x: -self.x,
                        y: -self.y,
                        _phantom: PhantomData,
                    }
                }
            }
        )*
    };
}
impl_neg_for_signed!(i8, i16, i32, i64, f32, f64);

impl<S, U> std::fmt::Display for Vector2<S, U>
where
    S: Integer + std::fmt::Display,
    U: Unit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        self.x.fmt(f)?;
        write!(f, ", ")?;
        self.y.fmt(f)?;
        write!(f, "]")?;

        Ok(())
    }
}

impl<I, U> From<Vector2<I, U>> for (I, I)
where
    I: Integer,
    U: Unit,
{
    fn from(value: Vector2<I, U>) -> Self {
        (value.x, value.y)
    }
}

impl<I, U> From<(I, I)> for Vector2<I, U>
where
    I: Integer,
    U: Unit,
{
    fn from(value: (I, I)) -> Self {
        Vector2::new(value.0, value.1)
    }
}
