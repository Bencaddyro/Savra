use crate::data::*;
use crate::cargo::*;

#[derive(Debug, Clone)]
pub struct State {
    pub wallet: usize,
    pub location: Location,
    pub haul: Cargo,
    pub time: f64,
}
