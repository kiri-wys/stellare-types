use std::marker::PhantomData;

use crate::math::{Decimal, Scalar};

pub trait AngleUnit {}
pub struct Radians;
pub struct Degrees;
impl AngleUnit for Radians {}
impl AngleUnit for Degrees {}

pub trait AngleConversions<D>
where
    D: Decimal + Scalar<D>,
{
    fn num_radians(self) -> D;
    fn num_degrees(self) -> D;

    fn to_radians(self) -> Angle<D, Radians>;
    fn to_degrees(self) -> Angle<D, Degrees>;
}

#[derive(Debug, Clone, Copy)]
pub struct Angle<D, U>
where
    D: Decimal,
    U: AngleUnit,
{
    num: D,
    _phantom: PhantomData<(D, U)>,
}
impl<D, U> Angle<D, U>
where
    D: Decimal + Scalar<D>,
    U: AngleUnit,
{
    pub fn radians(rad: D) -> Angle<D, Radians> {
        Angle {
            num: rad,
            _phantom: PhantomData,
        }
    }
    pub fn degrees(deg: D) -> Angle<D, Degrees> {
        Angle {
            num: deg,
            _phantom: PhantomData,
        }
    }
}
impl<D> AngleConversions<D> for Angle<D, Radians>
where
    D: Decimal,
{
    fn num_radians(self) -> D {
        self.num
    }

    fn num_degrees(self) -> D {
        self.num.to_degrees()
    }

    fn to_radians(self) -> Angle<D, Radians> {
        self
    }

    fn to_degrees(self) -> Angle<D, Degrees> {
        Angle {
            num: self.num_degrees(),
            _phantom: PhantomData,
        }
    }
}
impl<D> AngleConversions<D> for Angle<D, Degrees>
where
    D: Decimal,
{
    fn num_radians(self) -> D {
        self.num.to_radians()
    }

    fn num_degrees(self) -> D {
        self.num
    }

    fn to_radians(self) -> Angle<D, Radians> {
        Angle {
            num: self.num_degrees(),
            _phantom: PhantomData,
        }
    }

    fn to_degrees(self) -> Angle<D, Degrees> {
        self
    }
}
