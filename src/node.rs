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



type NodeDataRef = Arc<NodeData>;
type WeakNodeNodeRef = Weak<NodeData>;
type Parent = RwLock<WeakNodeNodeRef>;
type Children = RwLock<Vec<Child>>;
type Child = NodeDataRef;
type Score = RwLock<f64>;

pub struct NodeData {
  parent: Parent,
  children: Children,
  id: Uuid,
  action: Action,
  score: Score,
  state: Option<State>,
}
impl NodeData {

  pub fn location(self: &Self) -> Location {
    match &self.state {
      Some(s) => s.location,
      None => match self.action {
        Travel{location, ..} => location,
        _ => self.parent.read().unwrap().upgrade().unwrap().location(),
      }
    }
  }

  pub fn time(self: &Self) -> f64 {
    //println!("time {} <- {} ", self.id, self.parent.read().unwrap().upgrade().unwrap().id);
    match &self.state {
      Some(s) => s.time,
      None => match self.action {
        Travel{duration, ..} => duration + self.parent.read().unwrap().upgrade().unwrap().time(),
        Wait{duration} => duration + self.parent.read().unwrap().upgrade().unwrap().time(),
        _ => self.parent.read().unwrap().upgrade().unwrap().time(),
      }
    }
  }

  pub fn wallet(self: &Self) -> usize {
    //println!("wallet {} <- {} ", self.id, self.parentid());
    match &self.state {
      Some(s) => s.wallet,
      None => match self.action {
        Buy{amount, price, ..} => self.parent.read().unwrap().upgrade().unwrap().wallet() - (amount as f64 * price).ceil() as usize,
        Sell{amount, price, ..} => self.parent.read().unwrap().upgrade().unwrap().wallet() + (amount as f64 * price).ceil() as usize,
        _ => self.parent.read().unwrap().upgrade().unwrap().wallet(),
      }
    }
  }

  pub fn payload(&self) -> Payload {
    match &self.state {
      Some(s) => s.payload.clone(),
      None => match self.action {
        Buy{product, amount, ..} => self.parent.read().unwrap().upgrade().unwrap().payload().add(product, amount),
        Sell{product, amount, ..} => self.parent.read().unwrap().upgrade().unwrap().payload().remove(product, amount),
        _ => self.parent.read().unwrap().upgrade().unwrap().payload(),
      }
    }
  }

  pub fn market(&self) -> Market {
    match &self.state {
      Some(s) => s.market.clone(),
      None => self.parent.read().unwrap().upgrade().unwrap().market(),
    }
  }

  pub fn get_children(self: &Self) -> Vec<Arc<NodeData>> {
    self.children.read().unwrap().to_vec()
  }

  pub fn get_parent(self: &Self) -> Option<NodeDataRef> {
    let my_parent_weak = self.parent.read().unwrap();
    if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
      Some(my_parent_arc_ref)
    } else {
      None
    }
  }

  pub fn to_dot(self: &Self) -> String {
    let mut s = String::new();

    let id = self.id.to_simple();
    let wallet = self.wallet();
    let time = self.time();
    let location = self.location();
    let score = self.score.read().unwrap();
    let total = self.payload().capacity;
    let current = total - self.payload().empty();
    s.push_str(&format!("\t\"{id}\" [shape=record, label=\" {{ {wallet} ¤UEC | t={time} }} | {{ {location} | s={score:.3} }} | CARGO {current}/{total}"));

    for (product, amount) in self.payload().payload {
      s.push_str(&format!(" | {{ {product} | {amount} }}"));
    }

    s.push_str(&format!("\"];\n"));
    if let Some(parent) = self.get_parent() {
      let p_id = parent.id.to_simple();
      //let action = self.action;
      match self.action {
        Travel{location, duration, distance} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ MOVE | {duration}s }} | {{ {location} | {distance} }}\"];\n")),
        Buy{product, amount, price} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ BUY | {product} }} | {{ {amount} aSCU | {price} ¤UEC }}\"];\n")),
        Sell{product, amount, price} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ SELL | {product} }} | {{ {amount} aSCU | {price} ¤UEC }}\"];\n")),
        Wait{duration} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"WAIT | {duration}s\"];\n")),
      }
      s.push_str(&format!("\t\"{p_id}\" -> \"A{id}\""));
      s.push_str(&format!("\t\"A{id}\" -> \"{id}\";\n"));
    }
    s
  }
}

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



#[derive(Debug, Clone)]
pub struct Node {
  pub arc_ref: NodeDataRef,
}
impl Node
{
  pub fn root(wallet: usize, location: Location, capacity: usize) -> Node {
    let new_node = NodeData {
      id: Uuid::new_v4(),
      parent: RwLock::new(Weak::new()),
      children: RwLock::new(Vec::new()),
      state: Some(State{wallet, location, time: 0.0, payload: Payload{capacity, payload: HashMap::new()}, market: get_default_market()}),
      score: RwLock::new(0.0),
      action: Wait{duration: 0.0},
    };
    let arc_ref = Arc::new(new_node);
    Node { arc_ref }
  }

  pub fn child_action(action: Action) -> Node {
    let new_node = NodeData {
      id: Uuid::new_v4(),
      parent: RwLock::new(Weak::new()),
      children: RwLock::new(Vec::new()),
      state: None,
      score: RwLock::new(0.0),
      action,
    };
    let arc_ref = Arc::new(new_node);
    Node { arc_ref }
  }

  pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef {
    Arc::clone(&self.arc_ref)
  }

  pub fn create_and_add_child(self: &Self, action: Action) -> Node {
    let new_child = Node::child_action(action);
    self.add_child_and_update_its_parent(&new_child);
    new_child
  }

  pub fn update_score(self: &Self, time_limit: f64) {
    let mut wealth = self.wallet() as f64;
      if self.time() < time_limit {// add payload "max" value if not overtime
        for (p,a) in self.payload().payload {// to clean with map / sum
          wealth += p.max() * a as f64;
        }
      }
    let score: f64 = wealth / self.time();
    let mut value = self.arc_ref.score.write().unwrap();
    *value = score;
  }

  /// 🔏 Write locks used.
  pub fn add_child_and_update_its_parent(self: &Self, child: &Node) {
    {
      let mut my_children = self.arc_ref.children.write().unwrap();
      my_children.push(child.get_copy_of_internal_arc());
    } // `my_children` guard dropped.
    {
      let mut childs_parent = child.arc_ref.parent.write().unwrap();
      *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
    } // `my_parent` guard dropped.
  }

  pub fn has_parent(self: &Self) -> bool {
    self.get_parent().is_some()
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
}

impl Deref for Node
{
  type Target = NodeData;

  fn deref(&self) -> &Self::Target {
    &self.arc_ref
  }
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



