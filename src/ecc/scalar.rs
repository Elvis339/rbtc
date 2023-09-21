use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, BitAnd, Mul};

use crate::ecc::abstractions::{ArithmeticResult, FieldElementTrait};

#[derive(Debug, Clone)]
pub struct Scalar {
    pub n: BigInt,
}

/// `Scalar` is using binary expansion algorithm to repeatedly add point to itself `n` times.
impl Scalar {
    pub fn new(value: BigInt) -> Self {
        Self { n: value }
    }

    pub fn get_value(&self) -> &BigInt {
        &self.n
    }
}

impl From<u8> for Scalar {
    fn from(value: u8) -> Self {
        Self {
            n: BigInt::from(value),
        }
    }
}

impl From<u32> for Scalar {
    fn from(value: u32) -> Self {
        Self {
            n: BigInt::from(value),
        }
    }
}

impl<F: FieldElementTrait + Clone> Mul<&F> for Scalar {
    type Output = ArithmeticResult<F>;

    fn mul(self, rhs: &F) -> Self::Output {
        let prime = rhs.get_prime();
        let num = rhs.get_num();

        let mut coef = self.get_value() - &BigInt::one();
        let mut current = rhs.clone();
        let mut result = F::from_values(num.clone(), prime.clone())?;

        while coef > BigInt::zero() {
            if coef.clone().bitand(BigInt::one()) == BigInt::one() {
                let new_result = (result + &current)?;
                result = new_result;
            }
            let new_current = (current.clone() + current)?;
            current = new_current;
            coef >>= 1;
        }

        Ok(result)
    }
}

impl<'a, 'b, F: FieldElementTrait + Clone> Mul<&'b F> for &'a Scalar
where
    &'b F: Add<&'b F, Output = ArithmeticResult<F>>,
{
    type Output = ArithmeticResult<F>;

    fn mul(self, rhs: &'b F) -> Self::Output {
        let prime = rhs.get_prime();
        let num = rhs.get_num();

        let mut coef = self.n.clone() - BigInt::one(); // No need for a reference here
        let one = BigInt::one();
        let zero = BigInt::zero();

        let mut current = rhs.clone();
        let mut result = F::from_values(num.clone(), prime.clone())?;

        while &coef > &zero {
            if coef.clone().bitand(&one) == one {
                let new_result = (result + &current)?;
                result = new_result;
            }
            let new_current = (current.clone() + current)?;
            current = new_current;
            coef >>= 1;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;

    use super::*;

    fn new_fe(num: i64, prime: i64) -> FieldElement {
        FieldElement::new(num, prime).unwrap()
    }

    #[test]
    fn multiply_field_element() {
        let fe = new_fe(15, 223);
        let scalar = Scalar::from(2u8);

        let res_ref = (&scalar * &fe).unwrap();
        assert_eq!(res_ref, new_fe(30, 223));

        assert_eq!((scalar * &fe).unwrap(), new_fe(30, 223));
    }
}
