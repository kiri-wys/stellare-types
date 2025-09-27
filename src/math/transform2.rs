use std::{marker::PhantomData, ops::Mul};

use crate::math::{Angle, Decimal, Unit, Vector2, ViewSpace, WorldSpace};

#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub struct Affine2<D, F = (),T = () >
where
    D: Decimal,
    F: Unit,
    T: Unit,
{
    pub m00: D, pub m01: D,
    pub m10: D, pub m11: D,
    pub m20: D, pub m21: D,
    _phantom: PhantomData<(F, T)>,
}

impl<D, F, T> Affine2<D, F, T>
where
    D: Decimal,
    F: Unit,
    T: Unit,
{
    pub fn new(m00: D, m01: D, m10: D, m11: D, m20: D, m21: D) -> Self {
        Self {
            m00,
            m01,
            m10,
            m11,
            m20,
            m21,
            _phantom: PhantomData,
        }
    }
    #[rustfmt::skip]
    pub fn from_translation(translation: Vector2<D, T>) -> Self {
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
    pub fn from_scale(scale: D) -> Self {
        Self::from_nonuniform_scale(Vector2::new(scale, scale))
    }
    #[rustfmt::skip]
    pub fn from_nonuniform_scale(scale: Vector2<D, T>) -> Self {
        Self {
            m00: scale.x, m01: D::zero(),
            m10: D::zero(), m11: scale.y,
            m20: D::zero(), m21: D::zero(),
            _phantom: PhantomData,
        }
    }
}
impl<D> Affine2<D, WorldSpace, ViewSpace>
where
    D: Decimal,
{
    pub fn from_camera<A>(
        position: Vector2<D, WorldSpace>,
        rotation: A,
        zoom: D,
    ) -> Affine2<D, WorldSpace, ViewSpace>
    where
        A: Angle<D>,
    {
        let rads = rotation.radians();
        let (sin, cos) = rads.sin_cos();

        let m00 = cos / zoom;
        let m01 = sin / zoom;
        let m10 = -sin / zoom;
        let m11 = cos / zoom;
        let m20 = -(m00 * position.x + m01 * position.y);
        let m21 = -(m10 * position.x + m11 * position.y);

        Affine2 {
            m00,
            m01,
            m10,
            m11,
            m20,
            m21,
            _phantom: PhantomData,
        }
    }
}
impl<D, F, T, NT> Mul<Affine2<D, F, T>> for Affine2<D, T, NT>
where
    D: Decimal,
    F: Unit,
    T: Unit,
    NT: Unit,
{
    type Output = Affine2<D, F, NT>;

    fn mul(self, rhs: Affine2<D, F, T>) -> Self::Output {
        Affine2 {
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
impl<D, F, T> From<Affine2<D, F, T>> for [[D; 2]; 3]
where
    D: Decimal,
    F: Unit,
    T: Unit,
{
    fn from(value: Affine2<D, F, T>) -> Self {
        [
            [value.m00, value.m01],
            [value.m10, value.m11],
            [value.m20, value.m21],
        ]
    }
}
