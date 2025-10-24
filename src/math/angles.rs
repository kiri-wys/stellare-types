use std::{
    fmt::{Display, write},
    ops::Neg,
};

use stellare_types_derive::CwArithmetic;

use crate::math::Decimal;

pub trait Angle<D>
where
    D: Decimal,
{
    fn radians(self) -> D;
    fn degrees(self) -> D;
}

#[derive(Debug, Default, Clone, Copy, CwArithmetic)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Radians<D>(pub D)
where
    D: Decimal;
#[derive(Debug, Default, Clone, Copy, CwArithmetic)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Degrees<D>(pub D)
where
    D: Decimal;

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
