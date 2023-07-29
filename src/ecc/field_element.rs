use num_bigint::BigInt;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

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
    pub fn new(num: i64, prime: i64) -> Result<FieldElement, String> {
        if num >= prime || num < 0 {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        Ok(FieldElement {
            num: BigInt::from(num),
            prime: BigInt::from(prime),
        })
    }

    pub fn pow_mod(&self, exponent: BigInt) -> FieldElement {
        let mut n = exponent;
        let prime = &self.prime;
        while n < BigInt::from(0) {
            n += prime - 1;
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

// To satisfy `closed` property one of the tools we can use to make a finite field closed under addition,
// subtraction, multiplication, and division is modulo arithmetic.

/// a + b = (a + b) % p
impl Add for FieldElement {
    type Output = Result<FieldElement, String>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot add two numbers in different Fields.".to_string());
        }
        let prime = self.prime;
        let add = self.num + rhs.num;
        let num = add % prime.clone();
        Ok(FieldElement { num, prime })
    }
}

/// a - b = (a - b) % p
impl Sub for FieldElement {
    type Output = Result<FieldElement, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot subtract two numbers in different Fields.".to_string());
        }
        let prime = self.prime;
        let sub = self.num - rhs.num;
        let mut num = sub % prime.clone();

        if num < BigInt::from(0) {
            num += prime.clone();
        }

        Ok(FieldElement { num, prime })
    }
}

/// a * b = (a * b) % p
impl Mul for FieldElement {
    type Output = Result<FieldElement, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot multiply two numbers in different Fields.".to_string());
        }
        let prime = self.prime;
        let mul = self.num * rhs.num;
        let num = mul % prime.clone();
        Ok(FieldElement { num, prime })
    }
}

// p = 19
// 2/7 = 2*7^(19-2) = 2 * 7^17 = 465261027974414 % 19 = 3
impl Div for FieldElement {
    type Output = Result<FieldElement, String>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot multiply two numbers in different Fields.".to_string());
        }
        let prime = self.prime;
        let num = self.num * rhs.num.modpow(&prime.clone().sub(2), &prime) % prime.clone();
        Ok(FieldElement { num, prime })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_fe(num: i64, prime: i64) -> FieldElement {
        FieldElement::new(num, prime).unwrap()
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
    fn pow_test() {
        let prime = 31;
        let a = new_fe(17, prime.clone());
        assert_eq!(a.pow_mod(BigInt::from(3)), new_fe(15, prime.clone()));

        let b = new_fe(5, prime.clone());
        let c = new_fe(18, prime.clone());

        assert_eq!((b.pow_mod(BigInt::from(5)) * c).unwrap(), new_fe(16, prime));
    }

    #[test]
    fn pow_negative_test() {
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
}
