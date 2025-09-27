use std::ops::Neg;

use crate::math::Decimal;

pub trait Angle<D>
where
    D: Decimal,
{
    fn radians(self) -> D;
    fn degrees(self) -> D;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Radians<D: Decimal>(pub D);
#[derive(Debug, Default, Clone, Copy)]
pub struct Degrees<D: Decimal>(pub D);

impl<D> Angle<D> for Radians<D>
where
    D: Decimal,
{
    fn radians(self) -> D {
        self.0
    }

    fn degrees(self) -> D {
        self.0.to_degrees()
    }
}
impl<D> Angle<D> for Degrees<D>
where
    D: Decimal,
{
    fn radians(self) -> D {
        self.0.to_radians()
    }

    fn degrees(self) -> D {
        self.0
    }
}
impl<D> Neg for Radians<D>
where
    D: Decimal,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<D> Neg for Degrees<D>
where
    D: Decimal,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
