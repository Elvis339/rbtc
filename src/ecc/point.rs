use crate::ecc::field_element::FieldElement;
use num_bigint::BigInt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub fn new(
        a: FieldElement,
        b: FieldElement,
        x: Option<FieldElement>,
        y: Option<FieldElement>,
    ) -> Result<Point, String> {
        if x.is_none() && y.is_none() {
            return Ok(Self {
                a,
                b,
                x: None,
                y: None,
            });
        }

        let binding_x = x.clone().unwrap();
        let binding_y = y.clone().unwrap();

        // x^3 + a*x + b
        let curve = binding_x
            .pow_mod(BigInt::from(3))
            .add(a.clone().mul(binding_x.clone())?)?
            .add(b.clone())?;

        if binding_y.clone().pow_mod(BigInt::from(2)) != curve {
            return Err(format!(
                "({}, {}) is not on the curve!",
                x.unwrap(),
                y.unwrap()
            ));
        }

        return Ok(Point { a, b, x, y });
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        &self.x == &other.x && &self.y == &other.y && &self.a == &other.a && self.b == other.b
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.x.is_none() {
            return write!(f, "Point(infinity)");
        }

        let prime = self.a.get_prime();
        let y = self
            .y
            .clone()
            .map(|v| format!("FieldElement_{}({})", v.get_num(), v.get_prime()))
            .unwrap_or_else(|| format!("FieldElement_None({})", prime));

        return write!(
            f,
            "Point({}, {})_{}_{} FieldElement({})",
            &self.x.clone().unwrap(),
            y,
            &self.a,
            &self.b,
            prime,
        );
    }
}

impl Add for Point {
    type Output = Result<Point, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a && self.b != other.b {
            return Err(format!(
                "Points {}, {} are not on the same curve",
                self, other
            ));
        }

        let (_, prime) = self.a.clone().convert::<i64>()?;

        // Self is point at infinity
        if self.x.is_none() {
            return Ok(other);
        }

        // Other is point at infinity
        if other.x.is_none() {
            return Ok(self);
        }

        // Same x but different y (Additive inverse)
        if self.x == other.x && self.y != other.y {
            return Point::new(self.a, self.b, self.x, self.y);
        }

        // Different x
        // P1 + P2 = P3
        if self.x != other.x {
            let y1 = self.y.clone().expect("y1 is None");
            let y2 = other.y.clone().expect("y2 is None");

            let x1 = self.x.clone().expect("x1 is None");
            let x2 = other.x.clone().expect("x2 is None");

            // slope = (y2 - y1) / (x2 - x1)
            let slope = y2
                .clone()
                .sub(y1.clone())?
                .div(x2.clone().sub(x1.clone())?)?;

            // x3 = slope^2 - x1 - x2
            let x3 = slope
                .clone()
                .pow_mod(BigInt::from(2))
                .sub(x1.clone())?
                .sub(x2)?;

            // y3 = slope * (x1 - x3) - y1
            let y3 = slope.mul(x1.sub(x3.clone())?)?.sub(y1)?;

            return Point::new(self.a, self.b, Some(x3), Some(y3));
        }

        let zero = self
            .y
            .clone()
            .map(|y1| {
                let binding = self.x.clone().unwrap();
                let x = binding.get_num();
                *y1.get_num() == BigInt::from(0).mul(x)
            })
            .unwrap_or(false);

        // If we are tangent to the vertical line, we return point at infinity
        if self == other && zero {
            return Point::new(self.a, self.b, None, None);
        }

        // P1 + P1 = P2
        // Adding same point
        if self == other {
            let x1 = self.x.clone().expect("x1 is None");
            let y1 = self.y.clone().expect("y1 is None");

            if prime < 3 {
                return Err(format!("Prime {} too low!", prime));
            }

            let two = FieldElement::new(2, prime.clone())?;
            let three = FieldElement::new(3, prime.clone())?;

            // (3 * x1^2 + a)
            let quotient = three
                .mul(x1.clone().pow_mod(BigInt::from(2)))?
                .add(self.a.clone())?;

            // (2 * y1)
            let divident = two.clone().mul(y1.clone())?;

            // s = (3 * x1^2 + a) / (2 * y1)
            let s = quotient.div(divident)?;

            // x3 = s^2 - 2 * x1
            let x3 = s
                .clone()
                .pow_mod(BigInt::from(2))
                .sub(two)?
                .mul(x1.clone())?;

            // y3 = s * (x1 - x3) - y1
            let y3 = s.mul(x1.sub(x3.clone())?)?.sub(y1)?;

            return Point::new(self.a, self.b, Some(x3), Some(y3));
        }

        return Err(format!("Invalid"));
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;
    use crate::ecc::point::Point;

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
    }
}
