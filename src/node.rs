use std::collections::{HashSet, HashMap};
use std::{fmt, cmp};


use std::cmp::Ordering;
use std::sync::Arc;

use uuid::Uuid;

use crate::data::*;
use crate::action::*;

use crate::Node::{Root,Leaf};
use crate::action::Action::{Travel,Buy,Sell,Wait};

#[derive(Debug, Clone)]
pub enum Node {
    Root(RootNode),
    Leaf(LeafNode),
}
#[derive(Debug, Clone)]
pub struct RootNode {    
    id: Uuid,
    wallet: usize,
    location: Location,
    haul: Cargo,
    time_bound: f64,
}
#[derive(Debug, Clone)]
pub struct LeafNode {
    pub parent: Arc<Node>,
    id: Uuid,
    time: f64,
    action: Action,
}

#[derive(Debug, Clone)]
struct Cargo {
    capacity: usize,
    cargo: HashMap<Product,usize>,
}

impl Cargo {
    fn add(self, product: Product, amount: usize) -> Cargo {
        let mut c = self.clone();
        if let Some(current) = c.cargo.get_mut(&product) {
            *current += amount;
        }else{
            c.cargo.insert(product, amount);
        }
        c
    }
    
    fn remove(self, product: Product, amount: usize) -> Cargo {
        let mut c = self.clone();
        if let Some(current) = c.cargo.get_mut(&product) {
            *current -= amount;
            if *current == 0 { c.cargo.remove(&product); }
        }else{ assert!(false,"Error action sell but no amount in cargo !") }
        c
    }
    
    fn space(&self, product: Product) -> usize {
        let mut space = self.capacity * 100;
        for (p,a) in self.cargo.iter() {
            if product == *p {
                space -= a;
            } else {
                space -= 100 * (a / 100);
                if a % 100 > 0 { space -= 100 }            
            }
        }
        return space;
    }
}

impl Node {
    pub fn is_root(&self) -> bool { match self {
        Root(_) => true,
        Leaf(_) => false,
    }}

    pub fn parent(&self) -> Arc<Node> { match self {
        Root(_) => {println!("root access parent !"); Arc::new((*self).clone())},
        Leaf(n) => Arc::clone(&n.parent),
    }}

    pub fn location(&self) -> Location { match self {
        Root(n) => n.location,
        Leaf(n) => match n.action {
            Travel(l,_) => l,
            _ => n.parent.location()
        }
    }}
    
    pub fn time(&self) -> f64 { match self {
        Root(_) => 0.0,
        Leaf(n) => n.time,
    }}
    
    pub fn wallet(&self) -> usize { match self {
        Root(n) => n.wallet,
        Leaf(n) => match n.action {
            Buy(_,a,p) => n.parent.wallet() - (a as f64 * p).ceil() as usize,
            Sell(_,a,p) => n.parent.wallet() + (a as f64 * p).ceil() as usize,
            _ => n.parent.wallet()
        }
    }}

    pub fn action(&self) -> Action { match self {
        Root(_) => {println!("root access action !"); Wait(0.0)},
        Leaf(n) => n.action,
    }}

    pub fn id(&self) -> Uuid { match self {
        Root(n) => n.id,
        Leaf(n) => n.id,
    }}
    
    pub fn time_bound(&self) -> f64 { match self {
        Root(n) => n.time_bound,
        Leaf(n) => n.parent.time_bound(),
    }}
    
    fn score(&self) -> f64 { match self {
        Root(_) => 0.0,
        Leaf(_) => {
            let mut wealth = self.wallet() as f64;
            if self.time() < self.time_bound() {// add cargo "max" value if not overtime
                for (p,a) in &self.cargo().cargo {// to clean with map / sum
                    wealth += p.max() * *a as f64;
                }
            }
            wealth / self.time()
        }      
    }}
    
    fn cargo(&self) -> Cargo { match self {
        Root(n) => n.haul.clone(),
        Leaf(n) => match n.action {
            Buy(p,a,_) => n.parent.cargo().add(p,a),
            Sell(p,a,_) => n.parent.cargo().remove(p,a),
            _ => n.parent.cargo()
        }
    }}
    
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

}

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

pub fn root(wallet: usize, location: Location, capacity: usize, time_bound: f64) -> Node {
    Root(RootNode{
        id: Uuid::new_v4(),
        wallet,
        location,
        haul: Cargo{ capacity, cargo: HashMap::new() },
        time_bound,
    })
}

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



