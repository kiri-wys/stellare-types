use std::{marker::PhantomData, ops::Neg};

use stellare_types_derive::{BcArithmetic, BcBitops, CwArithmetic, CwBitops};

use crate::math::{Angle, Decimal, NormalizedVector2, Scalar, Unit, Vector};

pub type Vector2u<U> = Vector2<u32, U>;
pub type Vector2i<U> = Vector2<i32, U>;
pub type Vector2f<U> = Vector2<f32, U>;
pub type Vector2d<U> = Vector2<i64, U>;

#[derive(Debug, Default, Clone, Copy, CwArithmetic, CwBitops, BcArithmetic, BcBitops)]
pub struct Vector2<S, U = ()>
where
    S: Scalar,
    U: Unit,
{
    pub x: S,
    pub y: S,
    #[op_override("PhantomData")]
    _phantom: PhantomData<U>,
}
impl<S, U> Vector2<S, U>
where
    S: Scalar,
    U: Unit,
{
    pub fn new(x: S, y: S) -> Vector2<S, U> {
        Vector2 {
            x,
            y,
            _phantom: PhantomData,
        }
    }
}

impl<S, U> Vector<S> for Vector2<S, U>
where
    S: Scalar,
    U: Unit,
{
    type Precise = Vector2<S::Decimal, U>;
    type Normalized = Option<NormalizedVector2<S::Decimal, U>>;

    fn zero() -> Self {
        Self {
            x: S::zero(),
            y: S::zero(),
            _phantom: PhantomData,
        }
    }
    fn one() -> Self {
        Self {
            x: S::one(),
            y: S::one(),
            _phantom: PhantomData,
        }
    }
    fn splat(val: S) -> Self {
        Self::new(val, val)
    }

    fn to_precise(self) -> Self::Precise {
        Vector2 {
            x: self.x.to_precise(),
            y: self.y.to_precise(),
            _phantom: PhantomData,
        }
    }

    fn dot(self, other: Self) -> S {
        self.x * other.x + self.y * other.y
    }
    fn length_squared(self) -> S {
        self.x * self.x + self.y * self.y
    }
    fn length(self) -> S::Decimal {
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
    fn distance_to(self, other: Self) -> S::Decimal {
        (self - other).length()
    }
    fn distance_to_squared(self, other: Self) -> S {
        (self - other).length_squared()
    }

    fn rotate<A>(self, angle: A) -> Self::Precise
    where
        A: Angle<S::Decimal>,
    {
        let rads = angle.radians();
        let p = self.to_precise();
        Vector2 {
            x: p.x * rads.cos() - p.y * rads.sin(),
            y: p.x * rads.sin() + p.y * rads.cos(),
            _phantom: PhantomData,
        }
    }
    fn lerp(self, max: Self, alpha: S::Decimal) -> Self::Precise {
        let p_min = self.to_precise();
        let p_max = max.to_precise();
        Vector2 {
            x: p_min.x * (S::Decimal::one() - alpha) + p_max.x * alpha,
            y: p_min.y * (S::Decimal::one() - alpha) + p_max.y * alpha,
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
    fn min_component(self) -> (usize, S) {
        if self.x <= self.y {
            (0, self.x)
        } else {
            (1, self.y)
        }
    }
    fn max_component(self) -> (usize, S) {
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
