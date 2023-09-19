use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldElementError {
    FieldNotInRange(String),
    InvalidField(String),
    PointNotOnTheCurve(String),
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldElementError::FieldNotInRange(err) => {
                write!(f, "FieldNotInRange({})", err)
            }
            FieldElementError::InvalidField(err) => {
                write!(f, "InvalidField({})", err)
            }
            FieldElementError::PointNotOnTheCurve(err) => {
                write!(f, "PointNotOnTheCurve({})", err)
            }
        }
    }
}
