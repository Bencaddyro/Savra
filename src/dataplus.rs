use std::fmt;
use convert_case::{Case, Casing};
use crate::data::*;



impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self).to_case(Case::Title))
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self).to_case(Case::Title))
    }
}

