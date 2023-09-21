use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

use num_bigint::BigInt;
use num_traits::Zero;

use crate::ecc::abstractions::FieldElementTrait;
use crate::ecc::error::FieldElementError;
use crate::ecc::field_element::FieldElement;
use crate::ecc::scalar::Scalar;

// use crate::ecc::scalar::Scalar;

#[derive(Debug, Clone)]
pub struct Point<F: FieldElementTrait + Clone> {
    pub a: F,
    pub b: F,
    pub x: Option<F>, // Point at Infinity
    pub y: Option<F>, // Point at Infinity
}

impl<F: FieldElementTrait + Clone> Point<F> {
    pub fn new(a: F, b: F, x: Option<F>, y: Option<F>) -> Result<Point<F>, FieldElementError> {
        if x.is_none() || y.is_none() {
            return Ok(Self {
                a,
                b,
                x: None,
                y: None,
            });
        }
        let y_squared = &y
            .as_ref()
            .map(|y| {
                let exp = BigInt::from(2u8);
                y.pow_mod(exp)
            })
            .unwrap();

        let equation = &x
            .as_ref()
            .map(|x_value| {
                // x^3 + ax + b
                let exp = BigInt::from(3u8);
                let ax = a.clone().mul(x_value)?;
                x_value.pow_mod(exp).add(ax)?.add(b.clone())
            })
            .unwrap()
            .expect(format!("x={} is not on the curve", x.clone().unwrap()).as_str());

        if y_squared != equation {
            return Err(FieldElementError::PointNotOnTheCurve(format!(
                "({}, {}) is not on the curve",
                x.unwrap(),
                y.unwrap()
            )));
        }

        Ok(Self { a, b, x, y })
    }

    fn is_additive_inverse(&self, other: &Self) -> bool {
        self.x == other.x && self.y != other.y
    }

    fn is_on_vertical_line(&self) -> bool {
        if let Some(y1) = &self.y {
            if let Some(_) = &self.x {
                return *y1.get_num() == BigInt::zero();
            }
        }
        false
    }

    fn check_points_on_the_curve(&self, other: &Self) -> Result<(), FieldElementError> {
        if self.a != other.a && self.b != other.b {
            return Err(FieldElementError::PointNotOnTheCurve(format!(
                "Points {}, {} are not on the same curve",
                self, other
            )));
        }
        Ok(())
    }
}

impl<F: FieldElementTrait + Clone> PartialEq for Point<F> {
    fn eq(&self, other: &Self) -> bool {
        &self.x == &other.x && &self.y == &other.y && &self.a == &other.a && self.b == other.b
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl<F: FieldElementTrait + Clone> Display for Point<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.x.is_none() {
            return write!(f, "Point(infinity)");
        }

        let prime = self.a.get_prime();
        let y = self
            .y
            .as_ref()
            .map(|v| format!("FieldElement_{}({})", v.get_num(), v.get_prime()))
            .unwrap_or_else(|| format!("FieldElement_None({})", prime));

        return write!(
            f,
            "Point({}, {})_{}_{} FieldElement({})",
            self.x.as_ref().unwrap(),
            y,
            &self.a,
            &self.b,
            prime,
        );
    }
}

impl<F: FieldElementTrait + Clone> Add for Point<F> {
    type Output = Result<Point<F>, FieldElementError>;

    fn add(self, other: Self) -> Self::Output {
        self.check_points_on_the_curve(&other)?;

        // Self is point at infinity
        if self.x.is_none() {
            return Ok(other);
        }

        // Other is point at infinity
        if other.x.is_none() {
            return Ok(self);
        }

        // Same x but different y (Additive inverse)
        if self.is_additive_inverse(&other) {
            return Point::new(self.a, self.b, self.x, self.y);
        }

        // Different x
        // P1 + P2 = P3
        if self.x != other.x {
            let (x1, y1) = (self.x.as_ref().unwrap(), self.y.as_ref().unwrap());
            let (x2, y2) = (other.x.as_ref().unwrap(), other.y.as_ref().unwrap());

            // slope = (y2 - y1) / (x2 - x1)
            let slope = (y2.clone().sub(y1)? / x2.clone().sub(x1)?)?;

            // x3 = slope^2 - x1 - x2
            let x3 = &slope.pow_mod(BigInt::from(2u8)).sub(x1)?.sub(x2)?;

            // y3 = slope * (x1 - x3) - y1
            let y3 = &slope.mul(x1.clone().sub(x3)?)?.sub(y1)?;

            return Point::new(self.a, self.b, Some(x3.clone()), Some(y3.clone()));
        }

        // If we are tangent to the vertical line, we return point at infinity
        if self == other && self.is_on_vertical_line() {
            return Point::new(self.a, self.b, None, None);
        }

        // P1 + P1 = P2
        // Adding same point
        if self == other {
            let (x1, y1) = (self.x.as_ref().unwrap(), self.y.as_ref().unwrap());

            // 3 * x1^2 + a
            let quotient =
                Scalar::from(3u8).mul(&x1.pow_mod(BigInt::from(2)).add(self.a.clone())?)?;
            // 2 * y1
            let dividend = Scalar::from(2u8).mul(y1)?;

            let s = quotient.div(dividend)?;

            // x3 = s^2 - 2 * x1
            let x3 = s
                .clone()
                .pow_mod(BigInt::from(2))
                .sub(Scalar::from(2u8).mul(x1)?)?;

            // y3 = s * (x1 - x3) - y1
            let y3 = s.mul(x1.clone().sub(&x3)?)?.sub(y1)?;

            return Point::new(self.a, self.b, Some(x3.clone()), Some(y3));
        }

        return Err(FieldElementError::PointNotOnTheCurve(format!("Invalid")));
    }
}

impl<'a, 'b, F: FieldElementTrait + Clone> Add<&'b Point<F>> for &'a Point<F> {
    type Output = Result<Point<F>, FieldElementError>;

    fn add(self, other: &'b Point<F>) -> Self::Output {
        self.check_points_on_the_curve(&other)?;

        // Self is point at infinity
        if self.x.is_none() {
            return Ok(other.clone());
        }

        // Other is point at infinity
        if other.x.is_none() {
            return Ok(self.clone());
        }

        // Same x but different y (Additive inverse)
        if self.is_additive_inverse(&other) {
            return Point::new(
                self.a.clone(),
                self.b.clone(),
                self.x.clone(),
                self.y.clone(),
            );
        }

        // Different x
        // P1 + P2 = P3
        if self.x != other.x {
            let (x1, y1) = (self.x.as_ref().unwrap(), self.y.as_ref().unwrap());
            let (x2, y2) = (other.x.as_ref().unwrap(), other.y.as_ref().unwrap());

            // slope = (y2 - y1) / (x2 - x1)
            let slope = (y2.clone().sub(y1)? / x2.clone().sub(x1)?)?;

            // x3 = slope^2 - x1 - x2
            let x3 = &slope.pow_mod(BigInt::from(2u8)).sub(x1)?.sub(x2)?;

            // y3 = slope * (x1 - x3) - y1
            let y3 = &slope.mul(x1.clone().sub(x3)?)?.sub(y1)?;

            return Point::new(
                self.a.clone(),
                self.b.clone(),
                Some(x3.clone()),
                Some(y3.clone()),
            );
        }

        // If we are tangent to the vertical line, we return point at infinity
        if self == other && self.is_on_vertical_line() {
            return Point::new(self.a.clone(), self.b.clone(), None, None);
        }

        // P1 + P1 = P2
        // Adding same point
        if *self == *other {
            let (x1, y1) = (self.x.as_ref().unwrap(), self.y.as_ref().unwrap());

            // 3 * x1^2 + a
            let quotient =
                Scalar::from(3u8).mul(&x1.pow_mod(BigInt::from(2)).add(self.a.clone())?)?;
            // 2 * y1
            let dividend = Scalar::from(2u8).mul(y1)?;

            let s = quotient.div(dividend)?;

            // x3 = s^2 - 2 * x1
            let x3 = s
                .clone()
                .pow_mod(BigInt::from(2))
                .sub(Scalar::from(2u8).mul(x1)?)?;

            // y3 = s * (x1 - x3) - y1
            let y3 = s.mul(x1.clone().sub(&x3)?)?.sub(y1)?;

            return Point::new(self.a.clone(), self.b.clone(), Some(x3.clone()), Some(y3));
        }

        return Err(FieldElementError::PointNotOnTheCurve(format!("Invalid")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_fe(num: i64, prime: i64) -> FieldElement {
        FieldElement::new(num, prime).unwrap()
    }

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = new_fe(0, prime.clone());
        let b = new_fe(7, prime.clone());

        let valid_points = [(192, 105), (17, 56), (1, 193)];
        for v in valid_points {
            let x = new_fe(v.0, prime.clone());
            let y = new_fe(v.1, prime.clone());
            let p1 = Point::new(a.clone(), b.clone(), Some(x), Some(y));
            assert_eq!(p1.is_ok(), true,);
        }

        let invalid_points = [(200, 119), (42, 99)];
        for i in invalid_points {
            let x = new_fe(i.0, prime.clone());
            let y = new_fe(i.1, prime.clone());
            assert_eq!(
                Point::new(a.clone(), b.clone(), Some(x), Some(y)).is_err(),
                true
            );
        }
    }

    #[test]
    fn add_test() {
        // y^2 = x^3 - 7 over F-223
        let prime = 223;
        let a = new_fe(0, prime.clone());
        let b = new_fe(7, prime.clone());

        let additions = [
            //(x1, y2, x2, y2, x3, y3)
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];

        for item in additions {
            let x1 = new_fe(item.0, prime.clone());
            let y1 = new_fe(item.1, prime.clone());

            let x2 = new_fe(item.2, prime.clone());
            let y2 = new_fe(item.3, prime.clone());

            let x3 = new_fe(item.4, prime.clone());
            let y3 = new_fe(item.5, prime.clone());

            assert_eq!(
                (Point::new(a.clone(), b.clone(), Some(x1.clone()), Some(y1.clone())).unwrap()
                    + Point::new(a.clone(), b.clone(), Some(x2.clone()), Some(y2.clone()))
                        .unwrap())
                .unwrap(),
                Point::new(a.clone(), b.clone(), Some(x3.clone()), Some(y3.clone())).unwrap()
            );
        }

        let p1 = Point::new(
            a.clone(),
            b.clone(),
            Some(new_fe(192, prime.clone())),
            Some(new_fe(105, prime.clone())),
        )
        .unwrap();
        let p2 = Point::new(
            a.clone(),
            b.clone(),
            Some(new_fe(17, prime.clone())),
            Some(new_fe(56, prime.clone())),
        )
        .unwrap();
        assert_eq!(
            (&p1 + &p2).unwrap(),
            Point::new(
                a.clone(),
                b.clone(),
                Some(new_fe(170, prime.clone())),
                Some(new_fe(142, prime.clone())),
            )
            .unwrap()
        );
    }

    #[test]
    fn add_same_point() {
        let prime = 223;
        let a = new_fe(0, prime.clone());
        let b = new_fe(7, prime.clone());
        let x = new_fe(47, prime.clone());
        let y = new_fe(71, prime.clone());
        let p = Point::new(a.clone(), b.clone(), Some(x), Some(y)).unwrap();

        let result = (p.clone() + p.clone()).unwrap();

        assert_eq!(
            result,
            Point::new(
                a.clone(),
                b.clone(),
                Some(new_fe(36, prime.clone())),
                Some(new_fe(111, prime.clone())),
            )
            .unwrap()
        )
    }

    // #[test]
    // fn scalar_multiplication_point() {
    //     let prime = 223;
    //     let a = new_fe(0, prime.clone());
    //     let b = new_fe(7, prime.clone());
    //     let x = new_fe(47, prime.clone());
    //     let y = new_fe(71, prime.clone());
    //     let p = Point::new(a.clone(), b.clone(), Some(x), Some(y)).unwrap();
    //
    //     assert_eq!(
    //         Scalar::new(10) * p,
    //         Point::new(
    //             a,
    //             b,
    //             Some(new_fe(154, prime.clone())),
    //             Some(new_fe(150, prime.clone())),
    //         )
    //     )
    // }
}
