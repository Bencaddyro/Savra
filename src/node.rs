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
use crate::cargo::*;
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

  pub fn cargo(&self) -> Cargo {
    match &self.state {
      Some(s) => s.haul.clone(),
      None => match self.action {
        Buy{product, amount, ..} => self.parent.read().unwrap().upgrade().unwrap().cargo().add(product, amount),
        Sell{product, amount, ..} => self.parent.read().unwrap().upgrade().unwrap().cargo().remove(product, amount),
        _ => self.parent.read().unwrap().upgrade().unwrap().cargo(),
      }
    }
  }
}




#[derive(Debug, Clone)]
pub struct Node {
  arc_ref: NodeDataRef,
}
impl Node
{
  pub fn root(wallet: usize, location: Location, capacity: usize) -> Node {
    let new_node = NodeData {
      id: Uuid::new_v4(),
      parent: RwLock::new(Weak::new()),
      children: RwLock::new(Vec::new()),
      state: Some(State{wallet, location, time: 0.0, haul: Cargo{capacity, cargo: HashMap::new()}}),
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
      if self.time() < time_limit {// add cargo "max" value if not overtime
        for (p,a) in self.cargo().cargo {// to clean with map / sum
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

  /// 🔒 Read lock used.
  pub fn get_parent(self: &Self) -> Option<NodeDataRef> {
    let my_parent_weak = self.arc_ref.parent.read().unwrap();
    if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
      Some(my_parent_arc_ref)
    } else {
      None
    }
  }

  pub fn gen_children(self: &Self) -> Vec<Node> {
  // for now static, depending on node in futur version
    let map = get_map();
    let buy_table = get_buy();
    let sell_table = get_sell();
    let mut children: Vec<Node> = Vec::new();

    //try to move
    for (destination, distance) in map.get(&self.location()).unwrap() {
      let child: Node = self.create_and_add_child(Travel{location: *destination, duration: *distance, distance: *distance});
      children.push(child);
    }
    //try to buy something
    if buy_table.contains_key(&self.location()) {//location sell something
      for (product,price) in buy_table.get(&self.location()).unwrap() {
        let space = self.cargo().space(*product); //empty space in cargo
        let invest = (self.wallet() as f64 / *price).floor() as usize; //max invest capacity
        let amount = cmp::min(space, invest);
        if amount > 0 {
          let child = self.create_and_add_child(Buy{product: *product, amount, price: *price});
          children.push(child);
        }
      }
    }
    //try to sell something
    if (!self.cargo().cargo.is_empty()) & //cargo not empty,
       (sell_table.contains_key(&self.location())) {//location buy something
      let cargo_product: HashSet<Product> = self.cargo().cargo.keys().cloned().collect();//what we have
      let location_product: HashSet<Product> = sell_table[&self.location()].keys().cloned().collect();//what they buy
      for product in cargo_product.intersection(&location_product) {
        let amount = self.cargo().cargo[product];
        let price = sell_table[&self.location()][product];
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
      self.cargo(),
      "Action", self.action
      )
    }
}


/*
    pub fn dota(&self) -> String {
        match self.action() {
        Travel(location, distance) => format!("\"A{}\" [shape=Mrecord,label=\"{1} | {2}\"];\n",self.id().to_simple(), location, distance),
        Buy(product, amount, price) => format!("\"A{}\" [shape=Mrecord,label=\"{2} | {{ {1} aSCU | {3} ¤UEC }}\"];\n", self.id().to_simple(), amount, product, price),
        Sell(product, amount, price) => format!("\"A{}\" [shape=Mrecord,label=\"{2} | {{ {1} aSCU | {3} ¤UEC }}\"];\n", self.id().to_simple(), amount, product, price),
        Wait(time) => format!("\"A{}\" [shape=Mrecord,label=\"Wait {}s\"];\n", self.id().to_simple(),time),
        }
    }

    pub fn dot(&self) -> String {
        if self.is_root() {
            format!("\"{0}\" [shape=record,label=\"{{ {1}s | h={2:.3}}} | {3}¤UEC | {4}\"];",
            self.id().to_simple(), self.time(), self.score(), self.wallet(), self.location())
        } else {
            format!("\"{0}\" [shape=record,label=\"{{ {1}s | h={2:.3}}} | {3}¤UEC | {4}\"];\n{5}\"{6}\" -> \"A{0}\" -> \"{0}\";\n",
            self.id().to_simple(), self.time(), self.score(), self.wallet(), self.location(), self.dota(), self.parent().as_ref().id().to_simple())
        }
    }
    */
