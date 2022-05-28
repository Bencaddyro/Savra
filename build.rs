use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_yaml;
use convert_case::{Case, Casing};
use glob::glob;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Destination {
    location: String,
    distance: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Goods {
    product: String,
    price: f64,
    flow: Option<usize>,
    capacity: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    location: String,
    destination: Vec<Destination>,
    buy: Option<Vec<Goods>>,
    sell: Option<Vec<Goods>>,
}

fn read_yaml(path: &str) -> Vec<Entry> {
    let contents = fs::read_to_string(path).expect(&format!("Unable to read {}",path));
    serde_yaml::from_str::<Vec<Entry>>(&contents).expect(&format!("Err to parse {}",path))
}


fn write_data(file: &mut File, location: BTreeSet<String>, product_info: HashMap<String,InfoProd>, get_map: String, get_buy: String, get_sell: String) -> std::io::Result<()> {

    file.write(b"use std::collections::HashMap;\n")?;
    file.write(b"use strum::{EnumString,Display};\n")?;

    let product = product_info.keys().map(|x| x.to_owned()).collect::<Vec<String>>();
    
    // Enum Location
    file.write(b"\n// Auto-generated Location\n")?;
    file.write(b"#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, Display, EnumString)]\n")?;
    
    let s = format!("pub enum Location {{\n  {}\n}}",location.into_iter().collect::<Vec<String>>().join(",\n  "));
    file.write(s.as_bytes())?;
    
    // Enum Product
    file.write(b"\n\n// Auto-generated Product\n")?;
    file.write(b"#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, Display)]\n")?;
    //let s = format!("pub enum Product {{\n  {}\n}}", product_info.clone().into_keys().collect::<Vec<String>>().join(",\n  "));
    let s = format!("pub enum Product {{\n  {}\n}}", product.clone().join(",\n  "));
    file.write(s.as_bytes())?;
    
    // impl Product
    file.write(b"\n\nimpl Product {\n")?;
    
    // fn min / max
    let mut s_min = "pub fn min(&self) -> f64 {\nmatch self {\n".to_owned();
    let mut s_max = "pub fn max(&self) -> f64 {\nmatch self {\n".to_owned();
        
    for (p,InfoProd(min,max)) in &product_info {
        
        s_min += &format!("Product::{} => {} as f64,\n",p,min);
        s_max += &format!("Product::{} => {} as f64,\n",p,max);
    
    }
    //s_min += "_ => 0.0,\n}}\n";
    //s_max += "_ => 0.0,\n}}\n";
    
    s_min += "}}\n";
    s_max += "}}\n";
    
    file.write(s_min.as_bytes())?;
    file.write(s_max.as_bytes())?;
    
    // fn all_product
    file.write(format!("pub fn all() -> [Product;{}] {{\n",&product_info.len()).as_bytes())?;
    //let s = format!("[Product::{}\n]}}\n",product_info.clone().into_keys().collect::<Vec<String>>().join(",\nProduct::"));
    let s = format!("[Product::{}\n]}}\n",product.clone().join(",\nProduct::"));
    file.write(s.as_bytes())?;
    
    //end Enum Product
    file.write(b"}\n")?;
    
    // fn get_map
    file.write(b"\n\n// Auto-generated get_map()\n")?;
    file.write(get_map.as_bytes())?;
    // fn get_buy
    file.write(b"\n\n// Auto-generated get_buy()\n")?;
    file.write(get_buy.as_bytes())?;
    // fn get_sell
    file.write(b"\n\n// Auto-generated get_sell()\n")?;
    file.write(get_sell.as_bytes())?;
    
    file.write(b"\n\n")?;
    Ok(())
}

fn loc(s: String, v: f64) -> String { format!("(Location::{},{} as f64),",s,v) }
fn prod(s: String, v: f64) -> String { format!("(Product::{},{} as f64),",s,v) }

#[derive(Clone)]
struct InfoProd(f64,f64);


fn main() {

    // Info for source *.yml
    println!("cargo:rerun-if-changed=data_model");
    let input = "data_model/**/*.yml";
    
    // Import data from *.yml 
    let mut data = Vec::new();
    for entry in glob(input).unwrap() {
        if let Ok(path) = entry {
            let mut d = read_yaml(path.to_str().unwrap());
            data.append(&mut d);
        }
    }

    // Info for .rs output
    let output = "src/data.rs";
    let mut file = File::create(output).expect(&format!("Unable to open {}",output));
    
    // Enum set for Location & Product(min max)
    let mut location:BTreeSet<String> = BTreeSet::new();
    let mut location_control:BTreeSet<String> = BTreeSet::new();
    //let mut product:BTreeSet<InfoProd> = BTreeSet::new();
    let mut product_info:HashMap<String,InfoProd> = HashMap::new();
    
    // get header
    let mut get_map = "pub fn get_map() -> HashMap<Location,Vec<(Location,f64)>> {[\n".to_owned();
    let mut get_buy = "pub fn get_buy() -> HashMap<Location,HashMap<Product,f64>> {[\n".to_owned();
    let mut get_sell = "pub fn get_sell() -> HashMap<Location,HashMap<Product,f64>> {[\n".to_owned();
    
    for e in data {
        location.insert(e.location.clone().to_case(Case::Pascal));
        
        // get_map
        get_map.push_str(&format!("(Location::{},[",e.location.to_case(Case::Pascal)));
        for d in e.destination {
            location_control.insert(d.location.clone().to_case(Case::Pascal));
            get_map.push_str(&loc(d.location.to_case(Case::Pascal), d.distance));
        }
        get_map.push_str("].iter().cloned().collect()),\n");
        
        // get_buy
        if e.buy.is_some() {
            get_buy.push_str(&format!("(Location::{},[",e.location.to_case(Case::Pascal)));
            for g in e.buy.as_ref().unwrap() {
                get_buy.push_str(&prod(g.product.clone().to_case(Case::Pascal), g.price));

                // register / update new product
                let old = product_info.insert(g.product.clone().to_case(Case::Pascal),InfoProd(g.price,0.0));
                match old {
                    None => (),
                    Some(InfoProd(min,max)) => {product_info.insert(g.product.clone().to_case(Case::Pascal),InfoProd(g.price.min(min),max));()}
                }
                
            }
            get_buy.push_str("].iter().cloned().collect()),\n");
        }
        
        // get_sell
        if e.buy.is_some() {
            get_sell.push_str(&format!("(Location::{},[",e.location.to_case(Case::Pascal)));
            for g in e.sell.as_ref().unwrap() {
                get_sell.push_str(&prod(g.product.clone().to_case(Case::Pascal), g.price));
                
                // register / update new product
                let old = product_info.insert(g.product.clone().to_case(Case::Pascal),InfoProd(0.0,g.price));
                match old {
                    None => (),
                    Some(InfoProd(min,max)) => {product_info.insert(g.product.clone().to_case(Case::Pascal),InfoProd(min,g.price.max(max)));()}
                }
                                
            }
            get_sell.push_str("].iter().cloned().collect()),\n");
        }
    }
    
    // get footer
    get_map.push_str("].iter().cloned().collect()}");
    get_buy.push_str("].iter().cloned().collect()}");
    get_sell.push_str("].iter().cloned().collect()}");
    
    // Control Location closure
    assert!(location.symmetric_difference(&location_control).cloned().collect::<Vec<_>>().is_empty(), "Destination contains unknow location");
    
    // Write to output
    write_data(&mut file, location, product_info, get_map, get_buy, get_sell).unwrap();
    
}
