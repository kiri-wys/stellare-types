use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::math::{
    Angle, Decimal, NormalizedVector2, Radians, Scalar, Unit, Vector, angles::AngleConversions as _,
};

#[derive(Debug, Clone, Copy)]
pub struct Vector2<D, S, U = ()>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    pub x: S,
    pub y: S,
    _phantom: PhantomData<(D, U)>,
}
impl<D, S, U> Vector2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    pub fn new(x: S, y: S) -> Vector2<D, S, U> {
        Vector2 {
            x,
            y,
            _phantom: PhantomData,
        }
    }
}

impl<D, S, U> Vector<D, S> for Vector2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    type Precise = Vector2<D, D, U>;
    type Normalized = Option<NormalizedVector2<D, U>>;

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
    fn length(self) -> D {
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
    fn distance_to(self, other: Self) -> D {
        (self - other).length()
    }
    fn distance_to_squared(self, other: Self) -> S {
        (self - other).length_squared()
    }

    fn rotate(self, angle: Angle<D, Radians>) -> Self::Precise {
        let rads = angle.num_radians();
        let p = self.to_precise();
        Vector2 {
            x: p.x * rads.cos() - p.y * rads.sin(),
            y: p.x * rads.sin() + p.y * rads.cos(),
            _phantom: PhantomData,
        }
    }
    fn lerp(self, max: Self, alpha: D) -> Self::Precise {
        let p_min = self.to_precise();
        let p_max = max.to_precise();
        Vector2 {
            x: p_min.x * (D::one() - alpha) + p_max.x * alpha,
            y: p_min.y * (D::one() - alpha) + p_max.y * alpha,
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

impl<D, S, U> Add<Vector2<D, S, U>> for Vector2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    type Output = Self;

    fn add(self, rhs: Vector2<D, S, U>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            _phantom: PhantomData,
        }
    }
}
impl<D, S, U> Sub<Vector2<D, S, U>> for Vector2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    type Output = Self;

    fn sub(self, rhs: Vector2<D, S, U>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            _phantom: PhantomData,
        }
    }
}

impl<D, S, U> Mul<S> for Vector2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            _phantom: PhantomData,
        }
    }
}
impl<D, S, U> Div<S> for Vector2<D, S, U>
where
    D: Decimal,
    S: Scalar<D>,
    U: Unit,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            _phantom: PhantomData,
        }
    }
}
