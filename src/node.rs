use std::collections::{HashSet, HashMap};
use std::{fmt, cmp};


use std::cmp::Ordering;
use std::sync::Arc;

use uuid::Uuid;

use crate::data::*;
use crate::action::*;


pub struct Node {
    pub parent: Option<Arc<Node>>,
    //children: RefCell<Vec<Arc<Node>>>,
    
    pub id: Uuid,
    pub time: f64,
    pub wallet: usize,
    pub location: Location,
    
    pub cargo: HashMap<Product,usize>,
    capacity: usize,
    
    score: f64,
    pub action: Action,
    //pub state-price: bool,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node\t\t{}\nScore\t\t{}\nTime\t\t{}\nLocation\t{}\nWallet\t\t{}\nCargo\t\t{:?}\nAction\t\t{}\n",
                   self.id, self.score, self.time, self.location, self.wallet, self.cargo, self.action)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for Node {}


impl Node {

    //pub fn get_child(&self) -> Vec<Arc<Node>> { (*(self.children.borrow())).clone() }

    pub fn is_root(&self) -> bool {
        match self.parent {
            Some(_) => false,
            None => true,
        }
    }
    pub fn dota(&self) -> String {
        match self.action {
        Action::Travel(location, distance) => format!("\"A{}\" [shape=Mrecord,label=\"{1} | {2}\"];\n",self.id.to_simple(), location, distance),
        Action::Buy(product, amount, price) => format!("\"A{}\" [shape=Mrecord,label=\"{2} | {{ {1} aSCU | {3} ¤UEC }}\"];\n", self.id.to_simple(), amount, product, price),
        Action::Sell(product, amount, price) => format!("\"A{}\" [shape=Mrecord,label=\"{2} | {{ {1} aSCU | {3} ¤UEC }}\"];\n", self.id.to_simple(), amount, product, price),
        Action::Wait(time) => format!("\"A{}\" [shape=Mrecord,label=\"Wait {}s\"];\n", self.id.to_simple(),time),
        }
    }
    
    pub fn dot(&self) -> String {
        if self.is_root() {
            format!("\"{0}\" [shape=record,label=\"{{ {1}s | h={2:.3}}} | {3}¤UEC | {4}\"];",
            self.id.to_simple(), self.time, self.score, self.wallet, self.location)
        } else {
            format!("\"{0}\" [shape=record,label=\"{{ {1}s | h={2:.3}}} | {3}¤UEC | {4}\"];\n{5}\"{6}\" -> \"A{0}\" -> \"{0}\";\n",
            self.id.to_simple(), self.time, self.score, self.wallet, self.location, self.dota(), self.parent.as_ref().unwrap().id.to_simple())
        }
    }
    
    pub fn evaluate(&mut self, time_bound: f64){
        let mut wealth = self.wallet as f64;
        if self.time < time_bound {// add cargo "max" value if not overtime
            for (p,a) in &self.cargo {// to clean with map / sum
                wealth += p.max() * *a as f64;
            }
        }
        self.score = wealth / self.time;
    }
    
    pub fn travel(&mut self, location: Location, distance: f64){
        let speed = 1.0;
        self.wait(distance / speed);
        self.location = location;
    }
    
    pub fn buy(&mut self, product: Product, amount: usize, price: f64){
        //println!("wallet {}, amount {}, price {}, total {}",self.wallet, amount, price, (100.0 * amount as f64 * price).ceil() as usize);
        self.wallet -= (amount as f64 * price).ceil() as usize;
        if let Some(current) = self.cargo.get_mut(&product) {
            *current += amount;
        }else{
            self.cargo.insert(product, amount);
        }
        self.wait(1.0);
    }
    
    pub fn sell(&mut self, product: Product, amount: usize, price: f64){
        self.wallet += (amount as f64 * price).ceil() as usize;        
        if let Some(current) = self.cargo.get_mut(&product) {
            *current -= amount;
            if *current == 0 { self.cargo.remove(&product); }
        }else{ assert!(false,"Error action sell but no amount in cargo !") }
        self.wait(1.0);
    }
    
    pub fn wait(&mut self, time: f64){
        self.time += time;
    }
    
    fn space(&self, product: Product) -> usize {
        let mut space = self.capacity * 100;
        for (p,a) in self.cargo.iter() {
            if product == *p {
                space -= a;
            } else {
                space -= 100 * (a / 100);
                if a % 100 > 0 {
                space -= 100;
                }            
            }
        }
        return space;    
    }
}

pub fn root(wallet: usize, location: Location, capacity: usize) -> Node {
    Node{
        parent: None,
        //children: RefCell::new(vec![]),

        id: Uuid::new_v4(),
        time: 1.0,
        wallet,
        location,

        cargo: HashMap::new(),
        capacity,

        score: 0.0,
        action: Action::Wait(0.0),
        }
}

pub fn gen_children(node: Arc<Node>, time_bound: f64) -> Vec<Arc<Node>> {
    // for now static, depending on node in futur version
    let map = get_map();
    let buy_table = get_buy();
    let sell_table = get_sell();
    let mut children = Vec::new();
    
    //try to move
        for (destination, distance) in map.get(&node.location).unwrap() {
            let mut child = child_action(&node, Action::Travel(*destination, *distance));
            child.evaluate(time_bound);
            children.push(Arc::new(child));
    }
    //try to buy something
    if buy_table.contains_key(&node.location) {//location sell something
        for (product,price) in buy_table.get(&node.location).unwrap() {
            let space = node.space(*product); //empty space in cargo
            let invest = (node.wallet as f64 / *price).floor() as usize; //max invest capacity
            let amount = cmp::min(space, invest);
            if amount > 0 {
                let mut child = child_action(&node, Action::Buy(*product, amount, *price));
                child.evaluate(time_bound);
                children.push(Arc::new(child));
            }
        }
    }
    //try to sell something
    if (!node.cargo.is_empty()) & //cargo not empty,
        (sell_table.contains_key(&node.location)) {//location buy something
        let cargo_product: HashSet<Product> = node.cargo.keys().cloned().collect();//what we have
        let location_product: HashSet<Product> = sell_table[&node.location].keys().cloned().collect();//what they buy
        for product in cargo_product.intersection(&location_product) {
            let amount = node.cargo[product];
            let price = sell_table[&node.location][product];
            let mut child = child_action(&node, Action::Sell(*product, amount, price));
            child.evaluate(time_bound);
            children.push(Arc::new(child));
        }
    }
    children
}

fn child_action(node: &Arc<Node>, action: Action) -> Node {// assuming: action is legal !!!
    // create child node
    let mut child = Node{
        parent: Some(Arc::clone(node)),
        //children: RefCell::new(vec![]),
        
        id: Uuid::new_v4(),
        time: node.time,
        wallet: node.wallet,
        location: node.location,
        
        cargo: node.cargo.clone(),
        capacity: node.capacity,
        
        score: 0.0,
        action: action,
    };
    // apply action effect
    match action {
        Action::Travel(location, distance) => child.travel(location, distance),
        Action::Buy(product, amount, price) => child.buy(product, amount, price),
        Action::Sell(product, amount, price) => child.sell(product, amount, price),
        Action::Wait(time) => child.wait(time),
    }
    child
}



