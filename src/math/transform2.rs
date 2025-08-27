use std::{marker::PhantomData, ops::Mul};

use crate::math::{Angle, Decimal, Unit, Vector2};

#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct Affine2D<D, U = ()>
where
    D: Decimal,
    U: Unit,
{
    pub m00: D, pub m01: D,
    pub m10: D, pub m11: D,
    pub m20: D, pub m21: D,
    _phantom: PhantomData<U>,
}

impl<D, U> Affine2D<D, U>
where
    D: Decimal,
    U: Unit,
{
    #[rustfmt::skip]
    pub fn from_translation(translation: Vector2<D, D, U>) -> Self {
        Self {
            m00: D::one(), m01: D::zero(),
            m10: D::zero(), m11: D::one(),
            m20: translation.x, m21: translation.y,
            _phantom: PhantomData,
        }
    }
    #[rustfmt::skip]
    pub fn from_rotation<A>(rotation: A) -> Self where A: Angle<D> {
        let rads = rotation.radians();
        let (sin, cos) = rads.sin_cos();
        Self {
            m00: cos, m01: sin,
            m10: -sin, m11: cos,
            m20: D::zero(), m21: D::zero(),
            _phantom: PhantomData,
        }
    }
    #[rustfmt::skip]
    pub fn from_scale(scale: Vector2<D, D, U>) -> Self {
        Self {
            m00: scale.x, m01: D::zero(),
            m10: D::zero(), m11: scale.y,
            m20: D::zero(), m21: D::zero(),
            _phantom: PhantomData,
        }
    }
    pub fn from_uniform_scale(scale: D) -> Self {
        Self::from_scale(Vector2::new(scale, scale))
    }
}
impl<D, U> Mul for Affine2D<D, U>
where
    D: Decimal,
    U: Unit,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m00: self.m00 * rhs.m00 + self.m10 * rhs.m01,
            m01: self.m01 * rhs.m00 + self.m11 * rhs.m01,
            m10: self.m00 * rhs.m10 + self.m10 * rhs.m11,
            m11: self.m01 * rhs.m10 + self.m11 * rhs.m11,
            m20: self.m00 * rhs.m20 + self.m10 * rhs.m21 + self.m20,
            m21: self.m01 * rhs.m20 + self.m11 * rhs.m21 + self.m21,
            _phantom: PhantomData,
        }
    }
}
impl<D, U> From<Affine2D<D, U>> for [[D; 2]; 3]
where
    D: Decimal,
    U: Unit,
{
    fn from(value: Affine2D<D, U>) -> Self {
        [
            [value.m00, value.m01],
            [value.m10, value.m11],
            [value.m20, value.m21],
        ]
    }
}
