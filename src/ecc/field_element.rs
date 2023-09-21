use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;
use num_traits::{One, Zero};

use crate::ecc::error::FieldElementError;

pub type PrimeCheck = Result<FieldElement, FieldElementError>;

macro_rules! arithmetic_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for FieldElement {
            type Output = PrimeCheck;

            fn $method(self, rhs: Self) -> Self::Output {
                self.check_primes(&rhs)?;
                let op = self.num $op rhs.num;
                let num = op % &self.prime;
                Ok(FieldElement { num, prime: self.prime.clone() })
            }
        }

        impl $trait<&FieldElement> for FieldElement {
            type Output = PrimeCheck;

            fn $method(self, rhs: &FieldElement) -> Self::Output {
                self.check_primes(rhs)?;
                let op = self.num $op &rhs.num;
                let num = op % &self.prime;
                Ok(FieldElement { num, prime: self.prime.clone() })
            }
        }

        impl<'a, 'b> $trait<&'b FieldElement> for &'a FieldElement {
            type Output = PrimeCheck;

            fn $method(self, rhs: &'b FieldElement) -> Self::Output {
                self.check_primes(rhs)?;
                let op = &self.num $op &rhs.num;
                let num = op % &self.prime;
                Ok(FieldElement { num, prime: self.prime.clone() })
            }
        }
    };
}

/// Finite Field Definition
/// A finite field is defined as a finite set of numbers and two operations `+` and `*` and properties:
/// 1. Closed property; means if a and b are in the set, a + b and a * b are in the set.
/// 2. Additive identity; means that 0 exists and has the property a + 0 = a
/// 3. Multiplicative identity; means 1 exists and has the property a * 1 = a
/// 4. Additive inverse; means if a is in the set, -a is in the set,
/// which is defined as the value that makes a + (-a) = 0
#[derive(Debug, Clone)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

impl FieldElement {
    pub fn new(num: i64, prime: i64) -> Result<FieldElement, FieldElementError> {
        if num >= prime || num < 0 {
            return Err(FieldElementError::FieldNotInRange(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - 1
            )));
        }
        Ok(FieldElement {
            num: BigInt::from(num),
            prime: BigInt::from(prime),
        })
    }

    fn check_primes(&self, other: &FieldElement) -> Result<(), FieldElementError> {
        if self.prime != other.prime {
            return Err(FieldElementError::InvalidField(
                "Operands belong to different fields.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn get_num(&self) -> &BigInt {
        &self.num
    }

    pub fn get_prime(&self) -> &BigInt {
        &self.prime
    }

    pub fn pow_mod(&self, exponent: BigInt) -> FieldElement {
        let mut n = exponent;
        let prime = &self.prime;
        while n < BigInt::zero() {
            n += prime - BigInt::one();
        }
        let num = self.num.modpow(&n, &prime);
        FieldElement {
            num,
            prime: prime.clone(),
        }
    }

    pub fn convert<T: TryFrom<BigInt>>(&self) -> Result<(T, T), &'static str> {
        let num = T::try_from(self.num.clone()).map_err(|_| "Overflow while converting num!")?;
        let prime =
            T::try_from(self.prime.clone()).map_err(|_| "Overflow while converting prime!")?;

        Ok((num, prime))
    }

    pub fn construct_from(num: BigInt, prime: BigInt) -> Result<FieldElement, FieldElementError> {
        if num >= prime || num < BigInt::zero() {
            return Err(FieldElementError::FieldNotInRange(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - BigInt::one()
            )));
        }
        Ok(Self { num, prime })
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num.eq(&other.num) && self.prime.eq(&other.prime)
    }
}

arithmetic_op!(Add, add, +);
arithmetic_op!(Mul, mul, *);

impl Sub for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let sub = self.num - rhs.num;
        let mut num = sub % &self.prime;

        if num < BigInt::zero() {
            num += &self.prime;
        }

        Ok(FieldElement {
            num,
            prime: self.prime.clone(),
        })
    }
}

impl Sub<&FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn sub(self, rhs: &FieldElement) -> Self::Output {
        self.check_primes(rhs)?;
        let sub = &self.num - &rhs.num;
        let mut num = sub % &self.prime;

        if num < BigInt::zero() {
            num += &self.prime;
        }

        Ok(FieldElement {
            num,
            prime: self.prime.clone(),
        })
    }
}

impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn sub(self, rhs: &'b FieldElement) -> Self::Output {
        self.check_primes(rhs)?;
        let sub = &self.num - &rhs.num;
        let mut num = sub % &self.prime;

        if num < BigInt::zero() {
            num += &self.prime;
        }

        Ok(FieldElement {
            num,
            prime: self.prime.clone(),
        })
    }
}

// p = 19
// 2/7 = 2*7^(19-2) = 2 * 7^17 = 465261027974414 % 19 = 3
impl Div for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::InvalidField(
                "Cannot divide two numbers in different Fields.".to_string(),
            ));
        }
        let exp = &self.prime - BigInt::from(2u8);
        let num = self.num * rhs.num.modpow(&exp, &self.prime) % &self.prime;
        Ok(FieldElement {
            num,
            prime: self.prime.clone(),
        })
    }
}

impl Div<&FieldElement> for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn div(self, rhs: &FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::InvalidField(
                "Cannot divide two numbers in different Fields.".to_string(),
            ));
        }
        let exp = &self.prime - BigInt::from(2u8);
        let num = self.num * rhs.num.modpow(&exp, &self.prime) % &self.prime;
        Ok(FieldElement {
            num,
            prime: self.prime.clone(),
        })
    }
}

impl<'a, 'b> Div<&'b FieldElement> for &'a FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn div(self, rhs: &'b FieldElement) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::InvalidField(
                "Cannot divide two numbers in different Fields.".to_string(),
            ));
        }
        let exp = &self.prime - BigInt::from(2u8);
        let num = &self.num * rhs.num.modpow(&exp, &self.prime) % &self.prime;
        Ok(FieldElement {
            num,
            prime: self.prime.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_fe(num: i64, prime: i64) -> FieldElement {
        FieldElement::new(num, prime).unwrap()
    }

    #[test]
    fn err() {
        assert!(FieldElement::new(5, 3).is_err());
        assert!(FieldElement::new(3, 3).is_err());
        assert!((new_fe(2, 31) + new_fe(2, 7)).is_err());
        assert!((new_fe(2, 31) - new_fe(2, 7)).is_err());
        assert!((new_fe(2, 31) * new_fe(2, 7)).is_err());
    }

    #[test]
    fn equality_test() {
        let prime = 31;
        let a = new_fe(2, prime.clone());
        let b = new_fe(2, prime.clone());
        let c = new_fe(15, prime.clone());

        assert_eq!(a, b);
        assert_eq!(a != c, true);
        assert_eq!(a != b, false);
    }

    #[test]
    fn add_test() {
        let prime = 31;
        let a = new_fe(2, prime.clone());
        let b = new_fe(15, prime.clone());
        assert_eq!((a + b).unwrap(), new_fe(17, prime.clone()));

        let c = new_fe(17, prime.clone());
        let d = new_fe(21, prime.clone());
        assert_eq!((c + d).unwrap(), new_fe(7, prime));
    }

    #[test]
    fn sub_test() {
        let prime = 31;
        let a = new_fe(29, prime.clone());
        let b = new_fe(4, prime.clone());
        assert_eq!((a - b).unwrap(), new_fe(25, prime.clone()));

        let c = new_fe(15, prime.clone());
        let d = new_fe(30, prime.clone());
        assert_eq!((c - d).unwrap(), new_fe(16, prime));
    }

    #[test]
    fn mul_test() {
        let prime = 31;
        let a = new_fe(24, prime.clone());
        let b = new_fe(19, prime.clone());

        assert_eq!((a * b).unwrap(), new_fe(22, prime));
    }

    #[test]
    fn pow_mod_test() {
        let prime = 31;
        let a = new_fe(17, prime.clone());
        assert_eq!(a.pow_mod(BigInt::from(3u8)), new_fe(15, prime.clone()));

        let b = new_fe(5, prime.clone());
        let c = new_fe(18, prime.clone());

        assert_eq!(
            (b.pow_mod(BigInt::from(5u8)) * c).unwrap(),
            new_fe(16, prime)
        );
    }

    #[test]
    fn pow_mod_negative_test() {
        let prime = 31;
        let a = new_fe(17, prime.clone());
        assert_eq!(a.pow_mod(BigInt::from(-3)), new_fe(29, prime.clone()));

        let b = new_fe(4, prime.clone());
        let c = new_fe(11, prime.clone());

        assert_eq!(
            (b.pow_mod(BigInt::from(-4)) * c).unwrap(),
            new_fe(13, prime.clone())
        );
    }

    #[test]
    fn div_test() {
        let prime = 31;
        let a = new_fe(3, prime.clone());
        let b = new_fe(24, prime.clone());
        assert_eq!((a / b).unwrap(), new_fe(4, prime.clone()));
    }

    #[test]
    fn verify_point() {
        // y^2 = x^3 + 7 over finite field 103
        let prime = 103;
        let x = new_fe(17, prime.clone());
        let y = new_fe(64, prime.clone());

        // Verify: y2 = 64^2 % 103 = 79
        assert_eq!(y.pow_mod(BigInt::from(2)), new_fe(79, prime.clone()));

        // Verify: x^3 + 7 = (13^3 + 7) % 103 = 79
        assert_eq!(
            x.pow_mod(BigInt::from(3))
                .add(new_fe(7, prime.clone()))
                .unwrap(),
            new_fe(79, prime)
        )
    }
}
