use crate::data::*;
use crate::payload::*;

#[derive(Debug, Clone)]
pub struct State {
    pub wallet: usize,
    pub location: Location,
    pub haul: Payload,
    pub time: f64,
}
