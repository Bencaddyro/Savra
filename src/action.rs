use std::fmt;

use crate::data::*;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Travel(Location, f64),
    Buy(Product,usize,f64),
    Sell(Product,usize,f64),
    Wait(f64),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Travel(location, distance) => write!(f, "Travel -> {} ({})", location, distance),
            Action::Buy(product, amount, price) => write!(f, "Buy {}x{} at {}", amount, product, price),
            Action::Sell(product, amount, price) => write!(f, "Sell {}x{} at {}", amount, product, price),
            Action::Wait(time) => write!(f, "Wait for {}",time),
        }
    }
}


