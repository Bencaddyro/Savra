use crate::Location;
use crate::cargo::*;
use crate::action::Action;


#[derive(Debug, Clone)]
pub enum NodeContent {
    ValueAction,
    State,
}

#[derive(Debug, Clone)]
pub struct State {
    wallet: usize,
    location: Location,
    haul: Cargo,
    time: f64,
}

#[derive(Debug, Clone)]
pub struct ValueAction {
    time: f64,
    action: Action,
}

impl NodeContent {
    pub fn location(&self) -> Location { match self {
        State => self.location,
        ValueAction => self.action {
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
