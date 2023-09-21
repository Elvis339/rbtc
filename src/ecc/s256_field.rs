use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num_bigint::BigInt;
use num_traits::Pow;

use crate::ecc::abstractions::{ArithmeticResult, FieldElementTrait};
use crate::ecc::error::FieldElementError;
use crate::ecc::field_element::FieldElement;

/// `S256Field` concrete implementation of the FieldElement over prime field of 2**256 - 2**32 - 977
#[derive(Debug, Clone)]
pub struct S256Field {
    field: FieldElement,
}

impl FieldElementTrait for S256Field {
    fn get_num(&self) -> &BigInt {
        self.field.get_num()
    }

    fn get_prime(&self) -> &BigInt {
        self.field.get_prime()
    }

    fn from_values(num: BigInt, prime: BigInt) -> Result<Self, FieldElementError> {
        Ok(S256Field {
            field: FieldElement::from_values(num, prime)?,
        })
    }
}

impl S256Field {
    pub fn new(num: BigInt) -> S256Field {
        // 2^256 - 2^32 - 977
        let prime = BigInt::from(2u8)
            .pow(256u32)
            .sub(BigInt::from(2u8).pow(32u32))
            .sub(BigInt::from(977u32));

        Self {
            field: FieldElement::from_values(num, prime).expect("weird"),
        }
    }

    pub fn get_a() -> S256Field {
        S256Field::new(BigInt::from(0u8))
    }

    pub fn get_b() -> S256Field {
        S256Field::new(BigInt::from(7u8))
    }
}

impl fmt::Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "S256Field{}({})",
            self.field.get_prime(),
            self.field.get_num()
        )
    }
}

impl PartialEq for S256Field {
    fn eq(&self, other: &Self) -> bool {
        let num = self.field.get_num();
        let prime = self.field.get_prime();

        let other_num = other.field.get_num();
        let other_prime = other.field.get_prime();

        num.eq(other_num) && prime.eq(other_prime)
    }
}

impl Add for S256Field {
    type Output = ArithmeticResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_primes(&rhs)?;
        Ok(S256Field {
            field: self.field.add(rhs.field)?,
        })
    }
}

// impl<'a> Add<&'a S256Field> for S256Field {
//     type Output = ArithmeticResult<Self>;
//
//     fn add(self, rhs: &'a S256Field) -> Self::Output {
//         self.check_primes(&rhs)?;
//         Ok(S256Field {
//             field: self.field.add(&rhs.field)?,
//         })
//     }
// }

impl<'a, 'b> Add<&'b S256Field> for S256Field {
    type Output = ArithmeticResult<Self>;

    fn add(self, rhs: &'b Self) -> Self::Output {
        self.check_primes(rhs)?;
        let field = self.field.add(&rhs.field)?;
        Ok(S256Field { field })
    }
}

impl Sub for S256Field {
    type Output = ArithmeticResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let field = self.field.sub(rhs.field)?;
        Ok(S256Field { field })
    }
}

// impl<'a> Sub<&'a S256Field> for S256Field {
//     type Output = ArithmeticResult<Self>;
//
//     fn sub(self, rhs: &'a S256Field) -> Self::Output {
//         self.check_primes(&rhs)?;
//         Ok(S256Field {
//             field: self.field.sub(&rhs.field)?,
//         })
//     }
// }

impl<'a, 'b> Sub<&'b S256Field> for S256Field {
    type Output = ArithmeticResult<Self>;

    fn sub(self, rhs: &'b Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let field = self.field.sub(&rhs.field)?;
        Ok(S256Field { field })
    }
}

impl Mul for S256Field {
    type Output = ArithmeticResult<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let field = self.field.mul(rhs.field)?;
        Ok(S256Field { field })
    }
}

// impl<'a> Mul<&'a S256Field> for S256Field {
//     type Output = ArithmeticResult<Self>;
//
//     fn mul(self, rhs: &'a S256Field) -> Self::Output {
//         self.check_primes(&rhs)?;
//         Ok(S256Field {
//             field: self.field.mul(&rhs.field)?,
//         })
//     }
// }

impl<'a, 'b> Mul<&'b S256Field> for S256Field {
    type Output = ArithmeticResult<Self>;

    fn mul(self, rhs: &'b Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let field = self.field.mul(&rhs.field)?;
        Ok(S256Field { field })
    }
}

impl Div for S256Field {
    type Output = ArithmeticResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let field = self.field.div(rhs.field)?;
        Ok(S256Field { field })
    }
}

// impl<'a> Div<&'a S256Field> for S256Field {
//     type Output = ArithmeticResult<Self>;
//
//     fn div(self, rhs: &'a S256Field) -> Self::Output {
//         self.check_primes(&rhs)?;
//         Ok(S256Field {
//             field: self.field.div(&rhs.field)?,
//         })
//     }
// }

impl<'a, 'b> Div<&'b S256Field> for S256Field {
    type Output = ArithmeticResult<Self>;

    fn div(self, rhs: &'b Self) -> Self::Output {
        self.check_primes(&rhs)?;
        let field = self.field.div(&rhs.field)?;
        Ok(S256Field { field })
    }
}
