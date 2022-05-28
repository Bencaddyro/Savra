use std::{
  collections::HashMap,
  fmt,
};
use crate::Product;

#[derive(Debug, Clone)]
pub struct Cargo {
    pub capacity: usize,
    pub cargo: HashMap<Product,usize>,
}

impl Cargo {
    pub fn add(self, product: Product, amount: usize) -> Cargo {
        let mut c = self.clone();
        if let Some(current) = c.cargo.get_mut(&product) {
            *current += amount;
        }else{
            c.cargo.insert(product, amount);
        }
        c
    }
    
    pub fn remove(self, product: Product, amount: usize) -> Cargo {
        let mut c = self.clone();
        if let Some(current) = c.cargo.get_mut(&product) {
            *current -= amount;
            if *current == 0 { c.cargo.remove(&product); }
        }else{ assert!(false,"Error action sell but no amount in cargo !") }
        c
    }
    
    pub fn space(&self, product: Product) -> usize {
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

    pub fn empty(&self) -> usize {
        let mut space = self.capacity;
        for (_,a) in self.cargo.iter() {
            space -= 1 * (a / 100);
            if a % 100 > 0 { space -= 1 }
        }
        return space;
    }


}

impl fmt::Display for Cargo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut payload = String::new();
        for (p,a) in self.cargo.iter() {
            payload.push_str(&format!("--{:<25} {}\n",p,a));
        }
        write!(f, "Cargo\t\t{}/{}\n{}", self.empty(), self.capacity, payload)
    }
}
