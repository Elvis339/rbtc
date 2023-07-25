use num_bigint::BigInt;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug)]
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

        // x^3 + ax + b
        let curve_form = &x
            .clone()
            .unwrap_or(BigInt::from(0))
            .pow(3)
            .add(a.clone().mul(&x.clone().unwrap_or(BigInt::from(0))))
            .add(&b);

        // y^2 = x^3 + ax + b
        if &y.clone().unwrap_or(BigInt::from(0)).pow(2) != curve_form {
            return Err(format!(
                "Point(x={}, y={}) is not on the curve.",
                &x.unwrap(),
                &y.unwrap()
            ));
        }

        return Ok(Point { a, b, x, y });
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point={}={} + {} + {}",
            &self.y.clone().unwrap_or(BigInt::from(0)),
            &self.x.clone().unwrap_or(BigInt::from(0)).pow(3),
            self.a.clone().mul(&self.x.clone().unwrap_or(BigInt::from(1))),
            &self.b
        )
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.a.eq(&other.a) && self.b.eq(&other.b)
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
            "Point(x=5, y=7) is not on the curve.".to_string()
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
                format!("Point(x={}, y={}) is not on the curve.", n.0, n.1)
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
}
