use crate::ecc::error::FieldElementError;
use std::ops::Mul;

use crate::ecc::field_element::FieldElement;

#[derive(Debug, Clone)]
pub struct Scalar {
    value: i64,
}

impl Scalar {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

macro_rules! binary_exponentiation {
    ($lhs_value:expr, $rhs:expr) => {{
        let prime = $rhs.get_prime();
        let num = $rhs.get_num();

        let mut coef = $lhs_value - 1;
        let mut current = $rhs.clone();
        let mut result = FieldElement::construct_from(num.clone(), prime.clone())?;

        while coef > 0 {
            if coef & 1 == 1 {
                result = (&result + &current).unwrap();
            }
            current = (&current + &current).unwrap();
            coef >>= 1;
        }

        Ok(result)
    }};
}

impl Mul<FieldElement> for Scalar {
    type Output = Result<FieldElement, FieldElementError>;

    fn mul(self, rhs: FieldElement) -> Self::Output {
        binary_exponentiation!(self.value, rhs)
    }
}

impl Mul<&FieldElement> for Scalar {
    type Output = Result<FieldElement, FieldElementError>;

    fn mul(self, rhs: &FieldElement) -> Self::Output {
        binary_exponentiation!(self.value, rhs)
    }
}

impl<'a, 'b> Mul<&'b FieldElement> for &'a Scalar {
    type Output = Result<FieldElement, FieldElementError>;

    fn mul(self, rhs: &'b FieldElement) -> Self::Output {
        binary_exponentiation!(self.value, rhs)
    }
}

// impl Mul<Point> for Scalar {
//     type Output = Result<Point, FieldElementError>;
//
//     fn mul(self, rhs: Point) -> Self::Output {
//         let mut coef = self.value;
//         let mut current = rhs.clone();
//         let mut result = Point::new(rhs.a.clone(), rhs.b.clone(), None, None)?;
//
//         while coef > 0 {
//             if coef & 1 == 1 {
//                 result = (&result + &current).unwrap();
//             }
//             current = (&current + &current).unwrap();
//             coef >>= 1;
//         }
//
//         Ok(result)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn new_fe(num: i64, prime: i64) -> FieldElement {
        FieldElement::new(num, prime).unwrap()
    }

    #[test]
    fn multiply_field_element() {
        let fe = new_fe(15, 223);
        let scalar = Scalar::new(2);

        let res = (scalar.clone() * fe.clone()).unwrap();
        assert_eq!(res, new_fe(30, 223));

        let res_ref = (scalar.clone() * &fe).unwrap();
        assert_eq!(res_ref, new_fe(30, 223));

        let res_ref_ref = (&scalar * &fe).unwrap();
        assert_eq!(res_ref_ref, new_fe(30, 223));
    }
}
