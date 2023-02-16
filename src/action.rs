// ------------------------------------------------------------------------------------------------
/*
/   This file handle define Action type
*/
// ------------------------------------------------------------------------------------------------

use std::fmt;
use crate::data::*;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Travel{location: Location, duration: f64, distance: f64},
    Buy{product: Product, amount: usize, price: f64},
    Sell{product: Product, amount: usize, price: f64},
    Wait{duration: f64},
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Travel{location, duration, ..} => write!(f, "Travel -> {} ({})", location, duration),
            Action::Buy{product, amount, price} => write!(f, "Buy {}x{} at {}", amount, product, price),
            Action::Sell{product, amount, price} => write!(f, "Sell {}x{} at {}", amount, product, price),
            Action::Wait{duration} => write!(f, "Wait for {}",duration),
        }
    }
}
