// ------------------------------------------------------------------------------------------------
/*
/   This file contain various struct & function
*/
// ------------------------------------------------------------------------------------------------

use crate::data::*;
use crate::payload::*;
use crate::market::*;
use uuid::Uuid;
use std::{cmp::Ordering, fmt};
use convert_case::{Case, Casing};

// State

#[derive(Debug, Clone)]
pub struct State {
  pub wallet: usize,
  pub location: Location,
  pub payload: Payload,
  pub time: f64,
  pub market: Market,
}

impl State {
  pub fn score(self: &Self, time_limit: f64) -> f64 {
    let mut wealth = self.wallet as f64;
    if self.time < time_limit {// add payload "max" value if not overtime
      for (p,a) in &self.payload.payload {// to clean with map / sum
        wealth += p.max() * *a as f64;
      }
    }
    wealth / self.time
  }
}

// Action

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

// Data format

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

// Tag

pub struct Tag {
  pub id: Uuid,
  pub score: f64,
}
impl Ord for Tag {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score.partial_cmp(&other.score).unwrap()
  }
}
impl PartialOrd for Tag {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}
impl PartialEq for Tag {
  fn eq(&self, other: &Self) -> bool {
    self.score.eq(&other.score)
  }
}
impl Eq for Tag {}


