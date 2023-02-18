// ------------------------------------------------------------------------------------------------
/*
/   This file define node and various operation on it: most importantly valuation fonction and
/   child node generation
*/
// ------------------------------------------------------------------------------------------------

use core::fmt::Debug;
use std::{
  cmp,
  fmt,
  collections::{HashMap,HashSet,},
  sync::Arc,
};
use uuid::Uuid;

use crate::utils::*;
use crate::market::*;
use crate::payload::*;
use crate::data::*;
use crate::Action::{Travel,Buy,Sell,Wait};

#[derive(Debug)]
pub struct NewNode {
  parent: Uuid,
  id: Uuid,
  actions: Vec<Action>,
  score: f64,
  state: State,
}

pub type RefNewNode = Arc<NewNode>;


impl NewNode {

  pub fn location(self: &Self) -> Location { self.state.location }
  pub fn time(self: &Self) -> f64 { self.state.time }
  pub fn wallet(self: &Self) -> usize { self.state.wallet }
  pub fn payload(&self) -> &Payload { &self.state.payload }
  pub fn market(&self) -> &Market { &self.state.market }

  pub fn parent(self: &Self) -> Uuid { self.parent }
  pub fn id(self: &Self) -> Uuid { self.id }
  pub fn actions(self: &Self) -> &Vec<Action> { &self.actions }
  pub fn score(self: &Self) -> f64 { self.score }

  pub fn root(wallet: usize, location: Location, capacity: usize) -> NewNode {
    NewNode {
      parent: Uuid::nil(),
      id: Uuid::new_v4(),
      state: State{wallet, location, time: 0.0, payload: Payload{capacity, payload: HashMap::new()}, market: get_default_market()},
      score: 0.0,
      actions: Vec::new(),
    }
  }

  pub fn child_action(self: &Self, action: Action, time_limit: f64) -> RefNewNode {
    let mut state = self.state.clone();
    match action {
      Action::Travel{location, duration, ..} => { state.location = location; state.time += duration },
      Action::Buy{product, amount, price} => { state.payload = state.payload.add(product, amount); state.wallet -= (amount as f64*price).ceil() as usize },
      Action::Sell{product, amount, price} => { state.payload = state.payload.remove(product, amount); state.wallet += (amount as f64 *price).ceil() as usize },
      Action::Wait{duration} => { state.time += duration },
    };
    let mut actions = self.actions.clone();
    actions.insert(0, action);
    Arc::new(NewNode {
      parent: self.id,
      id: Uuid::new_v4(),
      state: state.clone(),
      score: state.score(time_limit),
      actions,
    })
  }

  pub fn gen_children(self: &Self, time_limit: f64) -> Vec<RefNewNode> {
  // for now static, depending on node in futur version
    let mut children: Vec<RefNewNode> = Vec::new();

    //try to move
    for (destination, distance) in self.location().get_destination() {
      let child: RefNewNode = self.child_action(Travel{location: destination, duration: distance, distance}, time_limit);
      children.push(child);
    }
    //try to buy something
    for product in self.location().get_product_buy() {
      let (price, _stock) = self.market()[&self.location()][&product];
      let space = self.payload().space(product); //empty space in payload
      let invest = (self.wallet() as f64 / price).floor() as usize; //max invest capacity
      let amount = cmp::min(space, invest);
      if amount > 0 {
        let child = self.child_action(Buy{product, amount, price}, time_limit);
        children.push(child);
      }
    }
    //try to sell something
    if !self.payload().payload.is_empty() { //payload not empty,
      let payload_product: HashSet<Product> = self.payload().payload.keys().cloned().collect();//what we have
      for product in payload_product.intersection(&self.location().get_product_sell()) { // Intersection what we have and what we can sell
        let amount = self.payload().payload[product];
        let (price, _stock) = self.market()[&self.location()][&product];
        let child = self.child_action(Sell{product: *product, amount, price}, time_limit);
        children.push(child);
      }
    }
    //wait until the end, do nothing
    let child: RefNewNode = self.child_action(Wait{duration: time_limit-self.time()}, time_limit);
    children.push(child);

    //get mut node from arc
    children
  }
}


impl fmt::Display for NewNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {}\n\
      {:<16}{:?}
      ",
      "Node", self.id,
      "Score", self.score,
      "Time", self.time(),
      "Location", self.location(),
      "Wallet", self.wallet(),
      self.payload(),
      "Action", self.actions
      )
    }
}
