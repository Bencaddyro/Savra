use std::collections::{HashSet, HashMap};
use std::{fmt, cmp};


use std::cmp::Ordering;
use std::sync::{Arc,Weak};

use uuid::Uuid;

use crate::data::*;
use crate::action::*;
use crate::cargo::*;
use crate::state::*;

//use crate::Node::{Root,Leaf};
use crate::action::Action::{Travel,Buy,Sell,Wait};

#[derive(Debug, Clone)]
pub struct Node {
    id: Uuid,
    parent: Weak<Node>,
    value: NodeContent,
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.parent.upgrade().is_none()
    }

    pub fn parent(&self) -> Weak<Node> {
        self.parent.clone()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    
    pub fn value(&self) -> NodeContent {
        self.value
    }        
}
/*
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node\t\t{}\nScore\t\t{}\nTime\t\t{}\nLocation\t{}\nWallet\t\t{}\nCargo\t\t{:?}\nAction\t\t{}\n",
                   self.id(), self.score(), self.time(), self.location(), self.wallet(), self.cargo(), self.action())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().partial_cmp(&other.score()).unwrap()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}
impl Eq for Node {}
*/
/*
impl Node {

    pub fn travel(&mut self, _location: Location, distance: f64){
        let speed = 1.0;
        self.wait(distance / speed);
    }
    
    pub fn buy(&mut self, _product: Product, _amount: usize, _price: f64){
        //println!("wallet {}, amount {}, price {}, total {}",self.wallet, amount, price, (100.0 * amount as f64 * price).ceil() as usize);
        self.wait(1.0);
    }
    
    pub fn sell(&mut self, _product: Product, _amount: usize, _price: f64){
        self.wait(1.0);
    }
  
    pub fn wait(&mut self, time: f64){ match self {
        Root(_) => (),
        Leaf(n) => n.time += time,
    }}
  
}
*/
pub fn root(wallet: usize, location: Location, capacity: usize, time_bound: f64) -> Node {
    Root{
        id: Uuid::new_v4(),
        parent: None,
        value: State{wallet,
                     location,
                     haul: Cargo{ capacity, cargo: HashMap::new() },
                     time_bound,
    }
}
/*
pub fn gen_children(node: Arc<Node>) -> Vec<Arc<Node>> {
    // for now static, depending on node in futur version
    let map = get_map();
    let buy_table = get_buy();
    let sell_table = get_sell();
    let mut children = Vec::new();
    
    //try to move
        for (destination, distance) in map.get(&node.location()).unwrap() {
            let child = child_action(&node, Travel(*destination, *distance));
            children.push(Arc::new(child));
    }
    //try to buy something
    if buy_table.contains_key(&node.location()) {//location sell something
        for (product,price) in buy_table.get(&node.location()).unwrap() {
            let space = node.cargo().space(*product); //empty space in cargo
            let invest = (node.wallet() as f64 / *price).floor() as usize; //max invest capacity
            let amount = cmp::min(space, invest);
            if amount > 0 {
                let child = child_action(&node, Buy(*product, amount, *price));
                children.push(Arc::new(child));
            }
        }
    }
    //try to sell something
    if (!node.cargo().cargo.is_empty()) & //cargo not empty,
        (sell_table.contains_key(&node.location())) {//location buy something
        let cargo_product: HashSet<Product> = node.cargo().cargo.keys().cloned().collect();//what we have
        let location_product: HashSet<Product> = sell_table[&node.location()].keys().cloned().collect();//what they buy
        for product in cargo_product.intersection(&location_product) {
            let amount = node.cargo().cargo[product];
            let price = sell_table[&node.location()][product];
            let child = child_action(&node, Sell(*product, amount, price));
            children.push(Arc::new(child));
        }
    }
    children
}

fn child_action(node: &Arc<Node>, action: Action) -> Node {// assuming: action is legal !!!
    // create child node
    let mut child = Leaf(LeafNode{
        parent: Arc::clone(node),
        id: Uuid::new_v4(),
        time: node.time(),
        action: action,
    });
    // apply action effect
    match action {
        Travel(location, distance) => child.travel(location, distance),
        Buy(product, amount, price) => child.buy(product, amount, price),
        Sell(product, amount, price) => child.sell(product, amount, price),
        Wait(time) => child.wait(time),
    }
    child
}

*/

