use num::pow;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct FieldElement {
    num: i64,
    prime: i64,
}

impl FieldElement {
    fn new(num: i64, prime: i64) -> Result<FieldElement, String> {
        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        Ok(FieldElement { num, prime })
    }

    pub fn pow(&self, exp: usize) -> Result<FieldElement, String> {
        if exp < 0 {
            return Err("Exponent cannot be negative!".to_string());
        }
        let num = pow(self.num.clone(), exp) % &self.prime;
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
        self.num == other.num && self.prime == other.prime
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
        let num = (self.num + rhs.num) % self.prime;
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
        let num = (self.num - rhs.num) % self.prime;
        FieldElement::new(num, self.prime.clone())
    }
}

/// a * b = (a * b) % p
impl Mul for FieldElement {
    type Output = Result<FieldElement, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err("Cannot subtract two numbers in different Fields.".to_string());
        }
        let num = (self.num * rhs.num) % self.prime;
        FieldElement::new(num, self.prime.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;

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
        let a = FieldElement::new(7, 19);
        let b = FieldElement::new(8, 19);

        assert_eq!(a.unwrap() + b.unwrap(), FieldElement::new(15, 19));
    }

    #[test]
    fn subtract() {
        let a = FieldElement::new(11, 19);
        let b = FieldElement::new(9, 19);

        assert_eq!(a.unwrap() - b.unwrap(), FieldElement::new(2, 19));
    }

    #[test]
    fn multiply() {
        let a = FieldElement::new(5, 19);
        let b = FieldElement::new(3, 19);

        assert_eq!(a.unwrap() * b.unwrap(), FieldElement::new(15, 19));
    }

    #[test]
    fn pow() {
        let a = FieldElement::new(7, 19);
        assert_eq!(a.unwrap().pow(3), FieldElement::new(1, 19))
    }
}
