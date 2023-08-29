use crate::ecc::field_element::FieldElement;
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

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;
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
}
