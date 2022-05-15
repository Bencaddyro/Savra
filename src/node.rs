use std::collections::{HashSet, HashMap};
//use std::collections::{HashMap};

use std::{fmt, cmp::*};
use std::cmp;


use std::sync::{Arc,Weak};

use uuid::Uuid;

use crate::data::*;
use crate::action::*;
use crate::cargo::*;
use crate::state::*;

use crate::action::Action::{Travel,Buy,Sell,Wait};

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    parent: Weak<Node>,
    action: Action,
    state: Option<State>,
    children: Vec<Arc<Node>>,
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.parent.upgrade().is_none()
    }

    fn parentid(&self) -> Uuid {
        let p = self.parent.upgrade();
        match p {
            Some(n) => n.id,
            None => self.id,
        }
    }

    pub fn parent(&self) -> Arc<Node> {
        let p = self.parent.upgrade();
        match p {
            Some(n) => n,
            None => {println!("Try to access parent but there is nothing !");
                     panic!("Panic")}
        }
    }

    fn location(&self) -> Location {
        match &self.state {
            Some(s) => s.location,
            None => match self.action {
                Travel{location, ..} => location,
                _ => self.parent().location(),
            }
        }
    }

    pub fn time(&self) -> f64 {
        println!("time {} <- {} ", self.id, self.parentid());
        match &self.state {
            Some(s) => s.time,
            None => match self.action {
                Travel{duration, ..} => duration + self.parent().time(),
                Wait{duration} => duration + self.parent().time(),
                _ => self.parent().time(),
            }
        }
    }

    fn wallet(&self) -> usize {
        println!("wallet {} <- {} ", self.id, self.parentid());
        match &self.state {
            Some(s) => s.wallet,
            None => match self.action {
                Buy{amount, price, ..} => self.parent().wallet() - (amount as f64 * price).ceil() as usize,
                Sell{amount, price, ..} => self.parent().wallet() + (amount as f64 * price).ceil() as usize,
                _ => self.parent().wallet(),
            }
        }
    }

    fn cargo(&self) -> Cargo {
        match &self.state {
            Some(s) => s.haul.clone(),
            None => match self.action {
                Buy{product, amount, ..} => self.parent().cargo().add(product, amount),
                Sell{product, amount, ..} => self.parent().cargo().remove(product, amount),
                _ => self.parent().cargo(),
            }
        }
    }

    fn score(&self) -> f64 {
        println!("score {} <- {} ", self.id, self.parentid());
        match &self.state {
            Some(s) => s.score,
            None => {
                let mut wealth = self.wallet() as f64;
                //if self.time() < self.time_bound() {// add cargo "max" value if not overtime
                    for (p,a) in &self.cargo().cargo {// to clean with map / sum
                        wealth += p.max() * *a as f64;
                    }
                //}
            wealth / self.time()
            },
        }
    }
}

pub fn get_children(node: Arc<Node>) -> Vec<Arc<Node>> {
        // for now static, depending on node in futur version
        let map = get_map();
        let buy_table = get_buy();
        let sell_table = get_sell();
        let mut children = Vec::new();

        //try to move
        for (destination, distance) in map.get(&node.location()).unwrap() {
            let child = new_child(node, Travel{location: *destination, duration: *distance, distance: *distance});
            children.push(Arc::new(child));
        }
        //try to buy something
        if buy_table.contains_key(&node.location()) {//location sell something
            for (product,price) in buy_table.get(&node.location()).unwrap() {
                let space = node.cargo().space(*product); //empty space in cargo
                let invest = (node.wallet() as f64 / *price).floor() as usize; //max invest capacity
                let amount = cmp::min(space, invest);
                if amount > 0 {
                    let child = new_child(node, Buy{product: *product, amount, price: *price});
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
                let child = new_child(node, Sell{product: *product, amount, price});
                children.push(Arc::new(child));
            }
        }
        //get mut node from arc
        node.children = children;
        children
    }

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        println!("cmp {} - {}", self.id, other.id);
        println!("------------------");

//         println!("{}",self);
        println!("------------------ * ");

        println!("{}",self.score());

        println!("------------------");
        println!("{}",other.id);
        println!("{:?}",other.parent);

        println!("------------------ ** ");

        println!("{}",other.score());

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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node\t\t{}\nParent\t\t{}\nScore\t\t{}\nTime\t\t{}\nLocation\t{}\nWallet\t\t{}\nCargo\t\t{:?}\nAction\t\t{}\n", self.id, self.parentid(), self.score(), self.time(), self.location(), self.wallet(), self.cargo(), self.action)
    }
}

pub fn root(wallet: usize, location: Location, capacity: usize) -> Node {
    Node{
        id: Uuid::new_v4(),
        parent: Weak::new(),
        state: Some(State{wallet, location, haul: Cargo{capacity, cargo: HashMap::new()}, time: 0.0, score: 0.0,}),
        action: Wait{duration: 0.0},
        children: Vec::new(),
    }
}

fn new_child(node: Arc<Node>, action: Action) -> Node {// assuming: action is legal !!!
    Node{
        parent: Arc::downgrade(&node),
        id: Uuid::new_v4(),
        action,
        state: None,
        children: Vec::new(),
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
