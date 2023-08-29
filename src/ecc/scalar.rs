use crate::ecc::field_element::FieldElement;
use crate::ecc::point::Point;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Scalar {
    value: i64,
}

impl Scalar {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Mul<FieldElement> for Scalar {
    type Output = Result<FieldElement, String>;

    // Naive impl
    fn mul(self, rhs: FieldElement) -> Self::Output {
        let prime = rhs.get_prime().clone();
        let num = rhs.get_num().clone();
        let mut result = FieldElement::construct_from(num.clone(), prime.clone()).unwrap();

        let end = self.value;

        for _ in 1..end {
            result = (result + rhs.clone())?
        }

        Ok(result)
    }
}

impl Mul<Point> for Scalar {
    type Output = Result<Point, String>;

    // Naive impl
    fn mul(self, rhs: Point) -> Self::Output {
        let mut result = Point::new(rhs.a.clone(), rhs.b.clone(), None, None).unwrap();

        let end = self.value;

        for _ in 0..end {
            result = (result + rhs.clone())?
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;
    use crate::ecc::point::Point;
    use crate::ecc::scalar::Scalar;

    fn new_fe(num: i64, prime: i64) -> FieldElement {
        FieldElement::new(num, prime).unwrap()
    }

    #[test]
    fn multiply_field_element() {
        let fe = new_fe(15, 223);
        let res = Scalar::new(2) * fe;
        assert_eq!(res.unwrap(), new_fe(30, 223))
    }

    #[test]
    fn scalar_multiplication_point() {
        let prime = 223;
        let a = new_fe(0, prime.clone());
        let b = new_fe(7, prime.clone());
        let x = new_fe(47, prime.clone());
        let y = new_fe(71, prime.clone());
        let p = Point::new(a.clone(), b.clone(), Some(x), Some(y)).unwrap();

        assert_eq!(
            Scalar::new(10) * p,
            Point::new(
                a,
                b,
                Some(new_fe(154, prime.clone())),
                Some(new_fe(150, prime.clone()))
            )
        )
    }
}
