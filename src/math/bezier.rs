use crate::math::{Decimal, Integer, Vector, Vector2};

#[derive(Debug, Clone, Copy, Default)]
pub struct CubicBezier<D>
where
    D: Decimal,
{
    pub p0: Vector2<D, ()>,
    pub p1: Vector2<D, ()>,
    pub p2: Vector2<D, ()>,
    pub p3: Vector2<D, ()>,
}

impl<D> CubicBezier<D>
where
    D: Decimal,
{
    pub fn point_at(&self, t: D) -> Vector2<D, ()> {
        // TODO: Computed 3 might not be the exact same as literal 3, account for that?
        let three = D::one() + D::one() + D::one();
        let u = D::one() - t;
        let uu = u * u;
        let uuu = uu * u;
        let tt = t * t;
        let ttt = tt * t;

        Vector2::new(
            uuu * self.p0.x
                + three * uu * t * self.p1.x
                + three * u * tt * self.p2.x
                + ttt * self.p3.x,
            uuu * self.p0.y
                + three * uu * t * self.p1.y
                + three * u * tt * self.p2.y
                + ttt * self.p3.y,
        )
    }
    pub fn derivative(&self, t: D) -> Vector2<D, ()> {
        // TODO: Computed 3 might not be the exact same as 3, account for that?
        let three = D::one() + D::one() + D::one();
        let six = three * (D::one() + D::one());
        let u = D::one() - t;

        ((self.p1 - self.p0) * u * u * three)
            + ((self.p2 - self.p1) * six * u * t)
            + ((self.p3 - self.p2) * three * t * t)
    }

    pub fn arc_lenght_by_simpsons<S>(&self, t: D, steps: S) -> D
    where
        S: Integer<Decimal = D>,
    {
        let stwo = S::one() + S::one();
        let two = D::one() + D::one();
        let three = D::one() + two;
        let four = two + two;
        let a = D::zero();
        let b = t;
        let h = (b - a) / steps.to_precise();
        let mut s = self.derivative(a).length() + self.derivative(b).length();

        let mut i = S::one();
        while i < steps {
            let x = a + i.to_precise() * h;
            let dx = self.derivative(x).length();

            s += if i % stwo == S::zero() {
                two * dx
            } else {
                four * dx
            };
            i += S::one();
        }
        s * h / three
    }

    pub fn find_t_for_length<S>(&self, length: D, steps: S, tolerance: D) -> D
    where
        S: Integer<Decimal = D>,
    {
        let total_length = self.arc_lenght_by_simpsons(D::one(), steps);
        let target = length.clamp(D::zero(), total_length);

        let mut low = D::zero();
        let mut high = D::one();

        let two = D::one() + D::one();
        let mut i = S::zero();
        while i < steps {
            let mid = (low + high) / two;
            let len = self.arc_lenght_by_simpsons(mid, steps);
            if len < target {
                low = mid;
            } else {
                high = mid;
            }
            i += S::one();
        }

        let mut t = (low + high) / two;
        for _ in 0..5 {
            let f = self.arc_lenght_by_simpsons(t, steps) - target;
            let dt = self.derivative(t).length();
            if t > tolerance {
                t -= f / dt;
            }
            t = t.clamp(D::zero(), D::one());
        }
        t
    }
}
