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

use crate::state::*;
use crate::market::*;
use crate::payload::*;
use crate::data::*;
use crate::action::*;
use crate::action::Action::{Travel,Buy,Sell,Wait};

#[derive(Debug)]
pub struct NewNode {
  id: Uuid,
  actions: Vec<Action>,
  score: f64,
  state: State,
}

type NewNodeRef = Arc<NewNode>;


impl NewNode {

  pub fn location(self: &Self) -> Location { self.state.location }
  pub fn time(self: &Self) -> f64 { self.state.time }
  pub fn wallet(self: &Self) -> usize { self.state.wallet }
  pub fn payload(&self) -> &Payload { &self.state.payload }
  pub fn market(&self) -> &Market { &self.state.market }

  pub fn id(self: &Self) -> Uuid { self.id }
  pub fn score(self: &Self) -> f64 { self.score }

  pub fn root(wallet: usize, location: Location, capacity: usize) -> NewNode {
    NewNode {
      id: Uuid::new_v4(),
      state: State{wallet, location, time: 0.0, payload: Payload{capacity, payload: HashMap::new()}, market: get_default_market()},
      score: 0.0,
      actions: Vec::new(),
    }
  }

  pub fn child_action(self: &Self, action: Action) -> NewNode {
    let mut state = self.state.clone();
    match action {
      Action::Travel{location, duration, ..} => { state.location = location; state.time += duration },
      Action::Buy{product, amount, price} => { state.payload = state.payload.add(product, amount); state.wallet -= (amount as f64*price).ceil() as usize },
      Action::Sell{product, amount, price} => { state.payload = state.payload.remove(product, amount); state.wallet += (amount as f64 *price).ceil() as usize },
      Action::Wait{duration} => { state.time += duration },
    };
    let mut actions = self.actions.clone();
    actions.insert(0, action);
    NewNode {
      id: Uuid::new_v4(),
      state: state.clone(),
      score: state.score(),
      actions,
    }
  }

  pub fn gen_children(self: &Self) -> Vec<NewNode> {
  // for now static, depending on node in futur version
    let mut children: Vec<NewNode> = Vec::new();

    //try to move
    for (destination, distance) in self.location().get_destination() {
      let child: NewNode = self.child_action(Travel{location: destination, duration: distance, distance});
      children.push(child);
    }
    //try to buy something
    for product in self.location().get_product_buy() {
      let (price, _stock) = self.market()[&self.location()][&product];
      let space = self.payload().space(product); //empty space in payload
      let invest = (self.wallet() as f64 / price).floor() as usize; //max invest capacity
      let amount = cmp::min(space, invest);
      if amount > 0 {
        let child = self.child_action(Buy{product, amount, price});
        children.push(child);
      }
    }
    //try to sell something
    if !self.payload().payload.is_empty() { //payload not empty,
      let payload_product: HashSet<Product> = self.payload().payload.keys().cloned().collect();//what we have
      for product in payload_product.intersection(&self.location().get_product_sell()) { // Intersection what we have and what we can sell
        let amount = self.payload().payload[product];
        let (price, _stock) = self.market()[&self.location()][&product];
        let child = self.child_action(Sell{product: *product, amount, price});
        children.push(child);
      }
    }
    //get mut node from arc
    children
  }

  // pub fn to_dot(self: &Self) -> String {
  //   let mut s = String::new();
  //
  //   let id = self.id.to_simple();
  //   let wallet = self.wallet();
  //   let time = self.time();
  //   let location = self.location();
  //   let score = self.score.read().unwrap();
  //   let total = self.payload().capacity;
  //   let current = total - self.payload().empty();
  //   s.push_str(&format!("\t\"{id}\" [shape=record, label=\" {{ {wallet} ¤UEC | t={time} }} | {{ {location} | s={score:.3} }} | CARGO {current}/{total}"));
  //
  //   for (product, amount) in self.payload().payload {
  //     s.push_str(&format!(" | {{ {product} | {amount} }}"));
  //   }
  //
  //   s.push_str(&format!("\"];\n"));
  //   if let Some(parent) = self.get_parent() {
  //     let p_id = parent.id.to_simple();
  //     //let action = self.action;
  //     match self.action {
  //       Travel{location, duration, distance} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ MOVE | {duration}s }} | {{ {location} | {distance} }}\"];\n")),
  //       Buy{product, amount, price} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ BUY | {product} }} | {{ {amount} aSCU | {price} ¤UEC }}\"];\n")),
  //       Sell{product, amount, price} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ SELL | {product} }} | {{ {amount} aSCU | {price} ¤UEC }}\"];\n")),
  //       Wait{duration} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"WAIT | {duration}s\"];\n")),
  //     }
  //     s.push_str(&format!("\t\"{p_id}\" -> \"A{id}\""));
  //     s.push_str(&format!("\t\"A{id}\" -> \"{id}\";\n"));
  //   }
  //   s
  // }

/*
  pub fn dota(self: &Self) -> String {
    match self.action() {
      Travel(location, distance) => format!("\"A{}\" [shape=Mrecord,label=\"{1} | {2}\"];\n",self.id().to_simple(), location, distance),
      Buy(product, amount, price) => format!("\"A{}\" [shape=Mrecord,label=\"{2} | {{ {1} aSCU | {3} ¤UEC }}\"];\n", self.id().to_simple(), amount, product, price),
      Sell(product, amount, price) => format!("\"A{}\" [shape=Mrecord,label=\"{2} | {{ {1} aSCU | {3} ¤UEC }}\"];\n", self.id().to_simple(), amount, product, price),
      Wait(time) => format!("\"A{}\" [shape=Mrecord,label=\"Wait {}s\"];\n", self.id().to_simple(),time),
    }
  }
*/

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



