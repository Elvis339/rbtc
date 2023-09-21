use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;
use num_traits::{One, Zero};

use crate::ecc::error::FieldElementError;

pub type ArithmeticResult<T> = Result<T, FieldElementError>;

pub trait FieldElementTrait:
    Sized
    + Display
    + PartialEq
    + Add<Output = ArithmeticResult<Self>>
    + for<'a> Add<&'a Self, Output = ArithmeticResult<Self>>
    + Sub<Output = ArithmeticResult<Self>>
    + for<'a> Sub<&'a Self, Output = ArithmeticResult<Self>>
    + Mul<Output = ArithmeticResult<Self>>
    + for<'a> Mul<&'a Self, Output = ArithmeticResult<Self>>
    + Div<Output = ArithmeticResult<Self>>
    + for<'a> Div<&'a Self, Output = ArithmeticResult<Self>>
{
    fn get_num(&self) -> &BigInt;
    fn get_prime(&self) -> &BigInt;
    fn from_values(num: BigInt, prime: BigInt) -> Result<Self, FieldElementError>;

    fn pow_mod(&self, exponent: BigInt) -> Self {
        let mut n = exponent;
        let prime: &BigInt = self.get_prime();
        while n < BigInt::zero() {
            n += prime - BigInt::one();
        }
        let num = self.get_num().modpow(&n, prime);
        Self::from_values(num, prime.clone()).expect("invalid params")
    }

    fn check_primes(&self, other: &Self) -> Result<(), FieldElementError> {
        if *self.get_prime() != *other.get_prime() {
            return Err(FieldElementError::InvalidField(
                "Operands belong to different fields.".to_string(),
            ));
        }
        Ok(())
    }
}
