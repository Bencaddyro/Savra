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
