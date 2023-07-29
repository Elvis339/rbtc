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
        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        Ok(FieldElement {
            num: BigInt::from(num),
            prime: BigInt::from(prime),
        })
    }

    pub fn pow(&self, exponent: BigInt) -> Result<FieldElement, String> {
        // Handle negative exponent
        // Fermat's little theorem holds true for every: a^{p-1} = 1
        // This fact means we can multiply by a^{p-1} as many times as we want
        let mut n = exponent;
        while n < BigInt::from(0) {
            n += &self.prime - 1
        }
        //
        let prime = self.prime.clone();
        let num = self.num.modpow(&n, &self.prime);
        Ok(FieldElement { num, prime })
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
        let num = self.num.add(rhs.num) % prime.clone();
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
        let num = self.num.sub(rhs.num) % prime.clone();
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
        let num = self.num.mul(rhs.num) % prime.clone();
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

        let exponent = prime.clone().sub(BigInt::from(2));
        let num = self.num.mul(rhs.num.pow(u32::try_from(exponent).unwrap())) % prime.clone();
        Ok(FieldElement { num, prime })
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;
    use num_bigint::BigInt;

    #[test]
    fn cannot_set_larger_num_then_prime() {
        let fe = FieldElement::new(10, 5);
        assert_eq!(fe.unwrap_err(), "Num 10 not in field range 0 to 4");
    }

    #[test]
    fn test_matching() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        assert_eq!(a == a, true);
        assert_eq!(a == b, false);
    }

    #[test]
    fn add() {
        let prime = 19;
        let a = FieldElement::new(7, prime);
        let b = FieldElement::new(8, prime.clone());

        assert_eq!(
            a.unwrap() + b.unwrap(),
            FieldElement::new(15, prime.clone())
        );
    }

    #[test]
    fn subtract() {
        let prime = 19;
        let a = FieldElement::new(11, prime.clone());
        let b = FieldElement::new(9, prime.clone());

        assert_eq!(a.unwrap() - b.unwrap(), FieldElement::new(2, prime));
    }

    #[test]
    fn multiply() {
        let prime = 19;
        let a = FieldElement::new(5, prime.clone());
        let b = FieldElement::new(3, prime.clone());

        assert_eq!(a.unwrap() * b.unwrap(), FieldElement::new(15, prime));
    }

    #[test]
    fn pow() {
        let prime = 19;
        let a = FieldElement::new(7, prime.clone());
        assert_eq!(a.unwrap().pow(BigInt::from(3)), FieldElement::new(1, prime));
    }

    #[test]
    fn modpow() {
        let prime = 13;
        let a = FieldElement::new(7, prime.clone());
        let b = FieldElement::new(8, prime.clone());
        assert_eq!(a.unwrap().pow(BigInt::from(-3)).unwrap() == b.unwrap(), true);
    }
}
