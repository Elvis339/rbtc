use num_bigint::BigInt;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Point {
    a: BigInt,
    b: BigInt,
    x: Option<BigInt>,
    y: Option<BigInt>,
}

impl Point {
    pub fn new(_a: i64, _b: i64, _x: Option<i64>, _y: Option<i64>) -> Result<Point, String> {
        let a = BigInt::from(_a);
        let b = BigInt::from(_b);

        if _x.is_none() && _y.is_none() {
            return Ok(Point {
                a,
                b,
                x: None,
                y: None,
            });
        }

        let x = _x.map(|v| BigInt::from(v));
        let y = _y.map(|v| BigInt::from(v));

        if y.is_none() {
            return Ok(Point { a, b, x, y: None });
        }

        return match x {
            Some(x_val) => {
                // x^3 + ax + b
                let curve_form = &x_val.pow(3).add(a.clone().mul(&x_val.clone())).add(&b);
                if &y.clone().unwrap().pow(2) != curve_form {
                    return Err(format!(
                        "Point(a={}, b={}, x={}, y={}) is not on the curve.",
                        a,
                        b,
                        x_val,
                        y.unwrap(),
                    ));
                }
                return Ok(Point {
                    a,
                    b,
                    x: Some(x_val),
                    y,
                });
            }
            None => Ok(Point { a, b, x: None, y }),
        };
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point({}={} + {} + {})",
            &self.y.clone().unwrap_or(BigInt::from(0)),
            &self.x.clone().unwrap_or(BigInt::from(0)).pow(3),
            self.a
                .clone()
                .mul(&self.x.clone().unwrap_or(BigInt::from(1))),
            &self.b
        )
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.a.eq(&other.a) && self.b.eq(&other.b)
    }
}

impl Add for Point {
    type Output = Result<Point, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            return Err(format!("{}, {} are not on the same curve.", self, rhs));
        }

        // If either is none
        // Some is the point at infinity, or the additive identity.
        if self.x.is_none() {
            return Ok(rhs);
        }

        if rhs.x.is_none() {
            return Ok(self);
        }
        //

        let x = self.x.clone().unwrap();
        let y = self.y.clone().unwrap();

        let other_x = rhs.x.clone().unwrap();
        let other_y = rhs.y.clone().unwrap();

        // Additive inverse (same x but different y, causing a vertical line)
        if x == y && y != other_y {
            if self.y.is_none() {
                return Ok(rhs);
            }

            if rhs.y.is_none() {
                return Ok(self);
            }
        }

        // Point addition when x1 != x2
        if x != other_x {
            let slope = other_y.sub(&y).div(&other_x.clone().sub(&x));
            let new_x = slope.pow(2).sub(&x).sub(&other_x);
            let new_y = slope.mul(&x.sub(&new_x)).sub(&y);
            return Ok(Point {
                a: self.a,
                b: self.b,
                x: Some(new_x),
                y: Some(new_y),
            });
        }

        return Err("Invalid point.".to_string());
    }
}

// Support &Point + &Point
impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Result<Point, String>;

    fn add(self, rhs: &'b Point) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            return Err(format!("{}, {} are not on the same curve.", self, rhs));
        }

        // If either is none
        if self.x.is_none() {
            return Ok(rhs.clone());
        }

        if rhs.x.is_none() {
            return Ok(self.clone());
        }

        // Additive inverse (same x but different y, causing a vertical line)
        if self.x.clone().unwrap() == rhs.x.clone().unwrap() {
            if self.y.is_none() {
                return Ok(rhs.clone());
            }

            if rhs.y.is_none() {
                return Ok(self.clone());
            }
        }

        return Err("Invalid point.".to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::point::Point;

    #[test]
    fn error_when_points_are_not_on_the_curve() {
        let p1 = Point::new(-1, -1, Some(5), Some(7));
        assert_eq!(
            p1.unwrap_err(),
            "Point(a=-1, b=-1, x=5, y=7) is not on the curve.".to_string()
        )
    }

    #[test]
    fn points_on_the_curve() {
        // y^2 = x^3 + 5x + 7
        let not = [(2, 4), (5, 7)];

        for n in not {
            let not_point = Point::new(5, 7, Some(n.0), Some(n.1));
            assert_eq!(
                not_point.unwrap_err(),
                format!(
                    "Point(a={}, b={}, x={}, y={}) is not on the curve.",
                    5, 7, n.0, n.1
                )
            );
        }

        let on = [(-1, -1), (18, 77)];
        for o in on {
            let x = o.0;
            let y = o.1;
            let point = Point::new(5, 7, Some(x.clone()), Some(y.clone()));
            assert_eq!(point.unwrap(), Point::new(5, 7, Some(x), Some(y)).unwrap())
        }
    }

    #[test]
    fn error_additive_identity() {
        let p1 = Point::new(-15, -1, Some(5), Some(7));
        let p2 = Point::new(0, -76, Some(5), Some(7));

        assert_eq!(
            p1.unwrap() + p2.unwrap(),
            Err(
                "Point(7=125 + -75 + -1), Point(7=125 + 0 + -76) are not on the same curve."
                    .to_string()
            )
        );
    }

    #[test]
    fn point_at_infinity() {
        // When x is None
        let p1 = Point::new(-15, -1, None, Some(7));
        let p2 = Point::new(-15, -1, Some(5), Some(7));

        assert_eq!(p1.as_ref().unwrap() + p2.as_ref().unwrap(), p2,);
    }

    #[test]
    fn additive_inverse() {
        // When p1.x == p2.x but y is None
        let p1 = Point::new(-15, -1, Some(5), Some(7));
        let p2 = Point::new(-15, -1, Some(5), None);

        assert_eq!(p1.as_ref().unwrap() + p2.as_ref().unwrap(), p1,);
    }

    #[test]
    // Addition when x1 != x2
    fn add_diff_xs() {
        let p1 = Point::new(5, 7, Some(2), Some(5));
        let p2 = Point::new(5, 7, Some(-1), Some(-1));

        assert_eq!(
            p1.unwrap() + p2.unwrap(),
            Point::new(5, 7, Some(3), Some(-7))
        );
    }
}
