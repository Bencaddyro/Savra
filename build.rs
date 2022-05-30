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
struct YamlDestination {
    location: String,
    distance: f64,
    bidirection: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct YamlCommodities {
    product: String,
    price: f64,
    flow: Option<usize>,
    capacity: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct YamlEntry {
    location: String,
    destination: Option<Vec<YamlDestination>>,
    buy: Option<Vec<YamlCommodities>>,
    sell: Option<Vec<YamlCommodities>>,
}

#[derive(Clone)]
struct InfoProd(f64,f64);


fn main() {
    println!("cargo:rerun-if-changed=data_model");

    // Import data from *.yml
    let input = "data_model/**/*.yml";
    let mut data: Vec<YamlEntry> = Vec::new();
    for entry in glob(input).unwrap() {
        if let Ok(path) = entry {
            let mut d = read_yaml(path.to_str().unwrap());
            data.append(&mut d);
        }
    }

    // Process data
    let mut location: BTreeSet<String> = BTreeSet::new();
    let mut location_control: BTreeSet<String> = BTreeSet::new();
    let mut product_info: HashMap<String, InfoProd> = HashMap::new();
    let mut get_map: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut get_buy: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let mut get_sell: HashMap<String, HashMap<String, f64>> = HashMap::new();
    process(data, &mut location, &mut location_control, &mut product_info, &mut get_map, &mut get_buy, &mut get_sell);

    // Check data
    assert!(location.symmetric_difference(&location_control).cloned().collect::<Vec<_>>().is_empty(), "Destination contains unknow location");

    // Write data
    let output = "src/data.rs";
    let mut file = File::create(output).expect(&format!("Unable to open {}",output));
    write_data(&mut file, location, product_info, get_map, get_buy, get_sell).unwrap();
}


fn read_yaml(path: &str) -> Vec<YamlEntry> {
    let contents = fs::read_to_string(path).expect(&format!("Unable to read {}",path));
    serde_yaml::from_str::<Vec<YamlEntry>>(&contents).expect(&format!("Err to parse {}",path))
}


fn process(data: Vec<YamlEntry>, location_enum: &mut BTreeSet<String>, location_control: &mut BTreeSet<String>, product_info: &mut HashMap<String, InfoProd>, get_map: &mut HashMap<String, HashMap<String, f64>>, get_buy: &mut HashMap<String, HashMap<String, f64>>, get_sell: &mut HashMap<String, HashMap<String, f64>>){
    for e in data {
        let location = e.location.to_case(Case::Pascal);

        // Register location entry
        location_enum.insert(location.clone());

        // Get destination
        if e.destination.is_some() {
            let mut data_map: HashMap<String,f64> = HashMap::new();
            for d in e.destination.unwrap_or_default() {
                let destination = d.location.to_case(Case::Pascal);
                location_control.insert(destination.clone());
                data_map.insert(destination.clone(), d.distance);

                // Handle bidirection entry
                if let Some(distance) = d.bidirection {
                    location_control.insert(location.clone());
                    if let Some(mut old_data) = get_map.insert(destination.clone(), HashMap::from([(location.clone(), distance)])) {
                        old_data.insert(location.clone(), distance);
                        get_map.insert(destination.clone(), old_data);
                    }
                }
            }
            get_map.insert(location.clone(), data_map);
        }

        // Get buy
        if e.buy.is_some() {
            let mut data_map: HashMap<String,f64> = HashMap::new();
            for g in e.buy.unwrap_or_default() {
                let product = g.product.to_case(Case::Pascal);
                data_map.insert(product.clone(), g.price);
                if let Some(InfoProd(min, max)) = product_info.insert(product.clone(), InfoProd(g.price,0.0)) {
                    product_info.insert(product.clone(), InfoProd(g.price.min(min), max));
                }
            }
            get_buy.insert(location.clone(), data_map);
        }

        // Get sell
        if e.sell.is_some() {
            let mut data_map: HashMap<String,f64> = HashMap::new();
            for g in e.sell.unwrap_or_default() {
                let product = g.product.to_case(Case::Pascal);
                data_map.insert(product.clone(), g.price);
                if let Some(InfoProd(min, max)) = product_info.insert(product.clone(), InfoProd(g.price,0.0)) {
                    product_info.insert(product.clone(), InfoProd(min, g.price.max(max)));
                }
            }
            get_sell.insert(location.clone(), data_map);
        }
    }
}


fn write_data(
    file: &mut File,
    location: BTreeSet<String>,
    product_info: HashMap<String,InfoProd>,
    get_map: HashMap<String, HashMap<String, f64>>,
    get_buy: HashMap<String, HashMap<String, f64>>,
    get_sell: HashMap<String, HashMap<String, f64>>
    ) -> std::io::Result<()> {

    // data.rs use
    file.write(b"use std::collections::HashMap;\n")?;
    file.write(b"use strum::EnumString;\n")?;

    // Enum Location
    file.write(b"\n// Auto-generated Location\n")?;
    file.write(b"#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, EnumString)]\n")?;
    file.write(b"pub enum Location {\n")?;
    for l in location {
        file.write(format!("\t{},\n", l).as_bytes())?;
    }
    file.write(b"}\n")?;

    // Enum Product
    file.write(b"\n// Auto-generated Product\n")?;
    file.write(b"#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, EnumString)]\n")?;
    file.write(b"pub enum Product {\n")?;
    for (p, ..) in product_info.clone() {
        file.write(format!("\t{},\n", p).as_bytes())?;
    }
    file.write(b"}\n")?;

    // Product impl
    file.write(b"\n// Auto-generated impl Product\n")?;
    file.write(b"impl Product {\n")?;

    // Product max
    file.write(b"\tpub fn max(&self) -> f64 {\n\t\tmatch self {\n")?;
    for (p, InfoProd(min,max)) in product_info {
        file.write(format!("\t\t\tProduct::{} => {} as f64,\n", p, max).as_bytes())?;
    }
    file.write(b"\t\t}\n\t}\n")?;

    file.write(b"}\n")?;



    // GET_MAP
    file.write(b"\n// // Auto-generated get_map()\n")?;
    file.write(b"pub fn get_map() -> HashMap<Location, HashMap<Location, f64>> {[\n")?;
    for (location, vec_destination) in get_map {
        file.write(format!("\t(Location::{}, [\n", location).as_bytes())?;
        for (destination, distance) in vec_destination {
            file.write(format!("\t\t(Location::{}, {} as f64),\n", destination, distance).as_bytes())?;
        }
        file.write(b"\t\t].iter().cloned().collect()),\n")?;
    }
    file.write(b"\t].iter().cloned().collect()}\n")?;

    // GET_BUY
    file.write(b"\n// // Auto-generated get_buy()\n")?;
    file.write(b"pub fn get_buy() -> HashMap<Location, HashMap<Product, f64>> {[\n")?;
    for (location, vec_product) in get_buy {
        file.write(format!("\t(Location::{}, [\n", location).as_bytes())?;
        for (product, price) in vec_product {
            file.write(format!("\t\t(Product::{}, {} as f64),\n", product, price).as_bytes())?;
        }
        file.write(b"\t\t].iter().cloned().collect()),\n")?;
    }
    file.write(b"\t].iter().cloned().collect()}\n")?;

    // GET_SELL
    file.write(b"\n// // Auto-generated get_sell()\n")?;
    file.write(b"pub fn get_sell() -> HashMap<Location, HashMap<Product, f64>> {[\n")?;
    for (location, vec_product) in get_sell {
        file.write(format!("\t(Location::{}, [\n", location).as_bytes())?;
        for (product, price) in vec_product {
            file.write(format!("\t\t(Product::{}, {} as f64),\n", product, price).as_bytes())?;
        }
        file.write(b"\t\t].iter().cloned().collect()),\n")?;
    }
    file.write(b"\t].iter().cloned().collect()}\n")?;
    Ok(())
}



