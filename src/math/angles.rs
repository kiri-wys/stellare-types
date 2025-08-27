use crate::math::Decimal;

pub trait Angle<D>
where
    D: Decimal,
{
    fn radians(self) -> D;
    fn degrees(self) -> D;
}

pub struct Radians<D: Decimal>(D);
pub struct Degrees<D: Decimal>(D);

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
