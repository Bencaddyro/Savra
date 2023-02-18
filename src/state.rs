// ------------------------------------------------------------------------------------------------
/*
/   This file define node state
*/
// ------------------------------------------------------------------------------------------------

use crate::data::*;
use crate::payload::*;
use crate::market::*;

#[derive(Debug, Clone)]
pub struct State {
    pub wallet: usize,
    pub location: Location,
    pub payload: Payload,
    pub time: f64,
    pub market: Market,
}

impl State {
    pub fn score(self: &Self) -> f64 {
        let mut wealth = self.wallet as f64;
      // if self.time() < time_limit {// add payload "max" value if not overtime
        // for (p,a) in self.payload().payload {// to clean with map / sum
          // wealth += p.max() * a as f64;
        // }
      // }
        wealth / self.time
    }
/*
    pub fn wallet(self: &Self, wallet: usize) -> State {
      State {
        wallet,
        location: self.location,
        payload: self.payload,
        time: self.time,
        market: self.market,
      }
    }
    pub fn location(self: &Self, location: Location) -> State {
      State {
        wallet: self.wallet,
        location,
        payload: self.payload,
        time: self.time,
        market: self.market,
      }
    }
    pub fn payload(self: &Self, payload: Payload) -> State {
      State {
        wallet: self.wallet,
        location: self.location,
        payload,
        time: self.time,
        market: self.market,
      }
    }
    pub fn time(self: &Self, time: f64) -> State {
      State {
        wallet: self.wallet,
        location: self.location,
        payload: self.payload,
        time,
        market: self.market,
      }
    }
    pub fn market(self: &Self, market: Market) -> State {
      State {
        wallet: self.wallet,
        location: self.location,
        payload: self.payload,
        time: self.time,
        market,
      }
    }*/
}

