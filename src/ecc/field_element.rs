use std::fmt;

#[derive(Debug)]
pub struct FieldElement {
    num: u128,
    prime: u128,
}

impl FieldElement {
    fn new(num: u128, prime: u128) -> Result<FieldElement, String> {
        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
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
        self.num == other.num && self.prime == other.prime
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
}
