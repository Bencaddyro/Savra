// ------------------------------------------------------------------------------------------------
/*
/   This file define node and various operation on it: most importantly valuation fonction and
/   child node generation
*/
// ------------------------------------------------------------------------------------------------

use core::fmt::Debug;
use std::{
  cmp,
  cmp::*,
  fmt,
  ops::Deref,
  sync::{Arc, RwLock, Weak},
  collections::{HashMap,HashSet,},
};
use uuid::Uuid;

use crate::state::*;
use crate::market::*;
use crate::payload::*;
use crate::data::*;
use crate::action::*;
use crate::action::Action::{Travel,Buy,Sell,Wait};


pub struct NewNode {
  children: Vec<Uuid>,
  id: Uuid,
  action: Vec<Action>,
  score: f64,
  state: State,
}

impl NewNode {

  pub fn location(self: &Self) -> Location { self.state.location }
  pub fn time(self: &Self) -> f64 { self.state.time }
  pub fn wallet(self: &Self) -> usize { self.state.wallet }
  pub fn payload(&self) -> Payload { self.state.payload }
  pub fn market(&self) -> Market { self.state.market }

  pub fn children(self: &Self) -> Vec<Uuid> { self.children } // Or Vec<NewNode> ? with hidden indirection from node index ?



  pub fn root(wallet: usize, location: Location, capacity: usize) -> NewNode {
    NewNode {
      id: Uuid::new_v4(),
      children: Vec!([]),
      state: State{wallet, location, time: 0.0, payload: Payload{capacity, payload: HashMap::new()}, market: get_default_market()}),
      score: 0.0,
      actions: Vec!([]),
    }
  }

  pub fn child_action(self: &Self, action: Action) -> Node {
    let mut state = self.state.clone();
    match action {
      Action::Travel{location, duration, ..} => { state.location = location; state.time += duration },
      Action::Buy{product, amount, price} => { state.payload = state.payload.add(product, amount); self.wallet -= amount*price },
      Action::Sell{product, amount, price} => { state.payload = state.payload.remove(product, amount); self.wallet += amount*price },
      Action::Wait{duration} => { state.time += duration },
    };
    NodeData {
      id: Uuid::new_v4(),
      children: Vec!([]),
      state: state,
      score: state.score(),
      actions: self.action.insert(0, action),
    }
  }

  pub fn gen_children(self: &Self) -> Vec<Node> {
  // for now static, depending on node in futur version
    let mut children: Vec<Node> = Vec::new();

    //try to move
    for (destination, distance) in self.location().get_destination() {
      let child: Node = self.create_and_add_child(Travel{location: destination, duration: distance, distance});
      children.push(child);
    }
    //try to buy something
    for product in self.location().get_product_buy() {
      let (price, _stock) = self.market()[&self.location()][&product];
      let space = self.payload().space(product); //empty space in payload
      let invest = (self.wallet() as f64 / price).floor() as usize; //max invest capacity
      let amount = cmp::min(space, invest);
      if amount > 0 {
        let child = self.create_and_add_child(Buy{product, amount, price});
        children.push(child);
      }
    }
    //try to sell something
    if !self.payload().payload.is_empty() { //payload not empty,
      let payload_product: HashSet<Product> = self.payload().payload.keys().cloned().collect();//what we have
      for product in payload_product.intersection(&self.location().get_product_sell()) { // Intersection what we have and what we can sell
        let amount = self.payload().payload[product];
        let (price, _stock) = self.market()[&self.location()][&product];
        let child = self.create_and_add_child(Sell{product: *product, amount, price});
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



impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.read().unwrap().partial_cmp(&other.score.read().unwrap()).unwrap()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score.read().unwrap().eq(&other.score.read().unwrap())
    }
}
impl Eq for Node {}


impl fmt::Debug for NodeData
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut parent_msg = String::new();
    if let Some(parent) = self.parent.read().unwrap().upgrade() {
      parent_msg.push_str(format!("📦 {}", parent.id).as_str());
    } else {
      parent_msg.push_str("🚫 None");
    }
    f.debug_struct("Node")
      .field("uuid", &self.id)
      .field("parent", &parent_msg)
      .field("children", &self.children)
      .finish()
  }
}

impl fmt::Display for NodeData {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let parentid = match self.parent.read().unwrap().upgrade() {
      Some(n) => n.id.to_string(),
      None => "None".to_string(),
    };
    write!(f, "\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {:<16}{}\n\
      {}\n\
      {:<16}{}
      ",
      "Node", self.id,
      "Parent", parentid,
      "Score", self.score.read().unwrap(),
      "Time", self.time(),
      "Location", self.location(),
      "Wallet", self.wallet(),
      self.payload(),
      "Action", self.action
      )
    }
}



