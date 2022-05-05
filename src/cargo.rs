use std::collections::HashMap;
use crate::Product;

#[derive(Debug, Clone)]
pub struct Cargo {
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
