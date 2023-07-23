use num_bigint::BigInt;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

impl FieldElement {
    fn new(num: BigInt, prime: BigInt) -> Result<FieldElement, String> {
        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        Ok(FieldElement { num, prime })
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
        let num = self.num.modpow(&n, &self.prime);
        FieldElement::new(num, self.prime.clone())
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
        let num = self.num.add(rhs.num) % self.prime.clone();
        FieldElement::new(num, self.prime.clone())
    }
}

/// a - b = (a - b) % p
impl Sub for FieldElement {
    type Output = Result<FieldElement, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot subtract two numbers in different Fields.".to_string());
        }
        let p = &self.prime;
        let num = self.num.sub(rhs.num) % p;
        FieldElement::new(num, self.prime)
    }
}

/// a * b = (a * b) % p
impl Mul for FieldElement {
    type Output = Result<FieldElement, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot multiply two numbers in different Fields.".to_string());
        }
        let p = &self.prime;
        let num = self.num.mul(rhs.num) % p;
        FieldElement::new(num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;
    use num_bigint::BigInt;

    #[test]
    fn cannot_set_larger_num_then_prime() {
        let fe = FieldElement::new(BigInt::from(10), BigInt::from(5));
        assert_eq!(fe.unwrap_err(), "Num 10 not in field range 0 to 4");
    }

    #[test]
    fn test_matching() {
        let a = FieldElement::new(BigInt::from(7), BigInt::from(13));
        let b = FieldElement::new(BigInt::from(6), BigInt::from(13));

        assert_eq!(a == a, true);
        assert_eq!(a == b, false);
    }

    #[test]
    fn add() {
        let a = FieldElement::new(BigInt::from(7), BigInt::from(19));
        let b = FieldElement::new(BigInt::from(8), BigInt::from(19));

        assert_eq!(
            a.unwrap() + b.unwrap(),
            FieldElement::new(BigInt::from(15), BigInt::from(19))
        );
    }

    #[test]
    fn subtract() {
        let a = FieldElement::new(BigInt::from(11), BigInt::from(19));
        let b = FieldElement::new(BigInt::from(9), BigInt::from(19));

        assert_eq!(
            a.unwrap() - b.unwrap(),
            FieldElement::new(BigInt::from(2), BigInt::from(19))
        );
    }

    #[test]
    fn multiply() {
        let a = FieldElement::new(BigInt::from(5), BigInt::from(19));
        let b = FieldElement::new(BigInt::from(3), BigInt::from(19));

        assert_eq!(
            a.unwrap() * b.unwrap(),
            FieldElement::new(BigInt::from(15), BigInt::from(19))
        );
    }

    #[test]
    fn pow() {
        let a = FieldElement::new(BigInt::from(7), BigInt::from(19));
        assert_eq!(
            a.unwrap().pow(BigInt::from(3)),
            FieldElement::new(BigInt::from(1), BigInt::from(19))
        );
    }

    #[test]
    fn modpow() {
        let a = FieldElement::new(BigInt::from(7), BigInt::from(13));
        let b = FieldElement::new(BigInt::from(8), BigInt::from(13));
        assert_eq!(
            a.unwrap().pow(BigInt::from(-3)).unwrap() == b.unwrap(),
            true
        );
    }
}
