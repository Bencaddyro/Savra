use std::{
  collections::HashMap,
  fmt,
};
use crate::Product;

#[derive(Debug, Clone)]
pub struct Payload {
    pub capacity: usize,
    pub payload: HashMap<Product,usize>,
}

impl Payload {
    pub fn add(self, product: Product, amount: usize) -> Payload {
        let mut c = self.clone();
        if let Some(current) = c.payload.get_mut(&product) {
            *current += amount;
        }else{
            c.payload.insert(product, amount);
        }
        c
    }
    
    pub fn remove(self, product: Product, amount: usize) -> Payload {
        let mut c = self.clone();
        if let Some(current) = c.payload.get_mut(&product) {
            *current -= amount;
            if *current == 0 { c.payload.remove(&product); }
        }else{ assert!(false,"Error action sell but no amount in payload !") }
        c
    }
    
    pub fn space(&self, product: Product) -> usize {
        let mut space = self.capacity * 100;
        for (p,a) in self.payload.iter() {
            if product == *p {
                space -= a;
            } else {
                space -= 100 * (a / 100);
                if a % 100 > 0 { space -= 100 }            
            }
        }
        return space;
    }

    pub fn empty(&self) -> usize {
        let mut space = self.capacity;
        for (_,a) in self.payload.iter() {
            space -= 1 * (a / 100);
            if a % 100 > 0 { space -= 1 }
        }
        return space;
    }


}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut payload = String::new();
        for (p,a) in self.payload.iter() {
            payload.push_str(&format!("--{:<25} {}\n",format!("{}",p),a));
        }
        write!(f, "Payload\t\t{}/{}\n{}", self.empty(), self.capacity, payload)
    }
}
