use std::fmt::{Display, Formatter};
use std::ops::{Add, BitAnd, Mul};

use num_bigint::BigInt;
use num_traits::{Num, One, Zero};

use crate::ecc::abstractions::FieldElementTrait;
use crate::ecc::error::FieldElementError;
use crate::ecc::point::point::Point;
use crate::ecc::s256_field::S256Field;
use crate::ecc::scalar::Scalar;

#[derive(Debug, Clone)]
pub struct S256Point {
    point: Point<S256Field>,
}

impl S256Point {
    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> Self {
        let a = S256Field::get_a();
        let b = S256Field::get_b();

        Self {
            point: Point::new(a, b, x, y).unwrap(),
        }
    }

    fn get_generator_point() -> S256Point {
        let gx = S256Field::new(
            BigInt::from_str_radix(
                "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                16,
            )
            .unwrap(),
        );

        let gy = S256Field::new(
            BigInt::from_str_radix(
                "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                16,
            )
            .unwrap(),
        );

        S256Point::new(Some(gx), Some(gy))
    }
}

impl Display for S256Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.point.x.is_none() {
            return write!(f, "S256Point(infinity)");
        }

        let prime = self.point.a.get_prime();
        let y = self
            .point
            .y
            .as_ref()
            .map(|v| format!("S256Field_{}({})", v.get_num(), v.get_prime()))
            .unwrap_or_else(|| format!("S256Field_None({})", prime));

        return write!(
            f,
            "S256Point({}, {})_{}_{} S256Field({})",
            self.point.x.as_ref().unwrap(),
            y,
            &self.point.a,
            &self.point.b,
            prime,
        );
    }
}

impl PartialEq for S256Point {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Add for S256Point {
    type Output = Result<S256Point, FieldElementError>;

    fn add(self, rhs: Self) -> Self::Output {
        let res = (self.point + rhs.point)?;
        Ok(S256Point { point: res })
    }
}

impl<'a, 'b> Add<&'b S256Point> for &'a S256Point {
    type Output = Result<S256Point, FieldElementError>;

    fn add(self, rhs: &'b S256Point) -> Self::Output {
        let res = (&self.point + &rhs.point)?;
        Ok(S256Point { point: res })
    }
}

impl Mul<&S256Point> for Scalar {
    type Output = Result<S256Point, FieldElementError>;

    fn mul(self, rhs: &S256Point) -> Self::Output {
        let mut coef = self.n;
        let one = BigInt::one();
        let zero = BigInt::zero();

        let mut current = rhs.clone();
        let mut result = S256Point::new(None, None);

        while coef > zero {
            if &coef.clone().bitand(&one) == &one {
                result = (&result + &current).unwrap();
            }
            current = (&current + &current).unwrap();
            coef >>= 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::ecc::point::s256_point::S256Point;
    use crate::ecc::s256_field::S256Field;
    use crate::ecc::scalar::Scalar;
    use num_bigint::BigInt;
    use num_traits::{Num, One, Pow};

    fn from_hex(input: &str) -> BigInt {
        BigInt::from_str_radix(input, 16).unwrap()
    }

    #[test]
    fn test_points() {
        // (private_key, x, y)
        let points = [
            (
                Scalar::new(BigInt::from(7u8)),
                from_hex("5cbdf0646e5db4eaa398f365f2ea7a0e3d419b7e0330e39ce92bddedcac4f9bc"),
                from_hex("6aebca40ba255960a3178d6d861a54dba813d0b813fde7b5a5082628087264da"),
            ),
            (
                Scalar::from(1485u32),
                from_hex("c982196a7466fbbbb0e27a940b6af926c1a74d5ad07128c82824a11b5398afda"),
                from_hex("7a91f9eae64438afb9ce6448a1c133db2d8fb9254e4546b6f001637d50901f55"),
            ),
            (
                Scalar::new(BigInt::from(2).pow(128u32)),
                from_hex("8f68b9d2f63b5f339239c1ad981f162ee88c5678723ea3351b7b444c9ec4c0da"),
                from_hex("662a9f2dba063986de1d90c2b6be215dbbea2cfe95510bfdf23cbf79501fff82"),
            ),
            (
                Scalar::new(BigInt::from(2).pow(240u32) + BigInt::from(2).pow(31u32)),
                from_hex("9577ff57c8234558f293df502ca4f09cbc65a6572c842b39b366f21717945116"),
                from_hex("10b49c67fa9365ad7b90dab070be339a1daf9052373ec30ffae4f72d5e66d053"),
            ),
        ];

        for point in points {
            let private_key = point.0;
            let x = S256Field::new(point.1);
            let y = S256Field::new(point.2);
            let pt = S256Point::new(Some(x), Some(y));

            assert_eq!(
                (private_key.clone() * &S256Point::get_generator_point()).unwrap(),
                pt.clone()
            );

            let invalid_key = Scalar::new(private_key.n - BigInt::one());
            assert_ne!(
                (invalid_key * &S256Point::get_generator_point()).unwrap(),
                pt
            )
        }
    }
}
