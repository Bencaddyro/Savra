use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_yaml;
use convert_case::{Boundary, Converter, Pattern};
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
    flux: Option<f64>,
    capacity: Option<usize>,
    stock: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct YamlEntry {
    location: String,
    destination: Option<Vec<YamlDestination>>,
    buy: Option<Vec<YamlCommodities>>,
    sell: Option<Vec<YamlCommodities>>,
}

#[derive(Clone)]
// InfoProd: minPrice maxPrice ?
struct InfoProd(f64, f64);

#[derive(Clone)]
// InfoFlux: Price, Capacity, Flux, Stock
struct InfoFlux(f64, usize, f64, usize);


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
    let mut location_destination: HashMap<String, HashMap<String, f64>> = HashMap::new();   // List all Location, Destination, distance
    let mut location_control: BTreeSet<String> = BTreeSet::new();                           // List all destination

    let mut product_info: HashMap<String, InfoProd> = HashMap::new();                       // Product info (min, max)

    let mut location_buy: HashMap<String, HashMap<String, InfoFlux>> = HashMap::new();      // Where do buy What (price, capacity, flux, stock)
    let mut location_sell: HashMap<String, HashMap<String, InfoFlux>> = HashMap::new();     // Where do sell What (price, capacity, flux, stock)

    process(data,
        &mut location_destination,
        &mut location_control,
        &mut product_info,
        &mut location_buy,
        &mut location_sell,
    );

    // Check data TODO
    //assert!(location.symmetric_difference(&location_control).cloned().collect::<Vec<_>>().is_empty(), "Destination contains unknow location");

    // Write data
    let output = "src/data.rs";
    let mut file = File::create(output).expect(&format!("Unable to open {}",output));
    write_data(&mut file,
        location_destination,
        product_info,
        location_buy,
        location_sell
    ).unwrap();
}


fn read_yaml(path: &str) -> Vec<YamlEntry> {
    let contents = fs::read_to_string(path).expect(&format!("Unable to read {}",path));
    serde_yaml::from_str::<Vec<YamlEntry>>(&contents).expect(&format!("Err to parse {}",path))
}


fn process(
    data: Vec<YamlEntry>,
    location_destination: &mut HashMap<String, HashMap<String, f64>>,
    location_control: &mut BTreeSet<String>,
    product_info: &mut HashMap<String, InfoProd>,
    location_buy: &mut HashMap<String, HashMap<String, InfoFlux>>,
    location_sell: &mut HashMap<String, HashMap<String, InfoFlux>>,
    ){

    // Custom Case Converter
    let custom_case = Converter::new()
        .set_boundaries(&[Boundary::Space,Boundary::Hyphen])
        .set_pattern(Pattern::Capital)
        .set_delim("");

    for e in data {
        let location = custom_case.convert(e.location);

        // Get destination
        if e.destination.is_some() {
            let mut data_map: HashMap<String,f64> = HashMap::new();
            for d in e.destination.unwrap_or_default() {
                let destination = custom_case.convert(d.location);
                location_control.insert(destination.clone());
                data_map.insert(destination.clone(), d.distance);

                // Handle bidirection entry
                if let Some(distance) = d.bidirection {
                    location_control.insert(location.clone());
                    if let Some(mut old_data) = location_destination.insert(destination.clone(), HashMap::from([(location.clone(), distance)])) {
                        old_data.insert(location.clone(), distance);
                        location_destination.insert(destination.clone(), old_data);
                    }
                }
            }
            location_destination.insert(location.clone(), data_map);
        }

        // Get buy
        if e.buy.is_some() {
            let mut info_buy: HashMap<String, InfoFlux> = HashMap::new();

            for g in e.buy.unwrap_or_default() {
                let product = custom_case.convert(g.product);

                // Price, Capacity, Flux, Stock
                info_buy.insert(product.clone(), InfoFlux(
                    g.price,
                    g.capacity.unwrap_or(1000),
                    g.flux.unwrap_or(500.0),
                    g.stock.unwrap_or(1000),
                    ));

                // Min & Max
                if let Some(InfoProd(min, max)) = product_info.insert(product.clone(), InfoProd(g.price,0.0)) {
                    product_info.insert(product.clone(), InfoProd(g.price.min(min), max));
                }

            }
            location_buy.insert(location.clone(), info_buy);

        }

        // Get sell
        if e.sell.is_some() {
            let mut info_sell: HashMap<String, InfoFlux> = HashMap::new();

            for g in e.sell.unwrap_or_default() {
                let product = custom_case.convert(g.product);

                // Price, Capacity, Flux, Stock
                info_sell.insert(product.clone(), InfoFlux(
                    g.price,
                    g.capacity.unwrap_or(1000),
                    g.flux.unwrap_or(500.0),
                    g.stock.unwrap_or(1000),
                ));

                // Min & Max
                if let Some(InfoProd(min, max)) = product_info.insert(product.clone(), InfoProd(g.price,0.0)) {
                    product_info.insert(product.clone(), InfoProd(min, g.price.max(max)));
                }
            }
            location_sell.insert(location.clone(), info_sell);
        }
    }
}


fn write_data(
    file: &mut File,
    location_destination: HashMap<String, HashMap<String, f64>>,
    product_info: HashMap<String,InfoProd>,
    location_buy: HashMap<String, HashMap<String, InfoFlux>>,
    location_sell: HashMap<String, HashMap<String, InfoFlux>>,
    ) -> std::io::Result<()> {

    // data.rs use
    file.write(b"use std::collections::{HashSet, HashMap};\n")?;
    file.write(b"use strum::EnumString;\n")?;
    file.write(b"use std::iter::FromIterator;\n")?;

    // --------------------------------------------------------------------------------------
    // Enum Location
    file.write(b"\n// Auto-generated Location\n")?;
    file.write(b"#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, EnumString)]\n")?;
    file.write(b"pub enum Location {\n")?;
    for (l, _) in &location_destination {
        file.write(format!("\t{},\n", l).as_bytes())?;
    }
    file.write(b"}\n")?;

    // Location Get All
    file.write(b"pub fn get_all_location() -> HashSet<Location> { HashSet::from_iter(vec!(\n")?;
    for (l, _) in &location_destination {
        file.write(format!("\tLocation::{},\n", l).as_bytes())?;
    }
    file.write(b"))}\n")?;

    // --------------------------------------------------------------------------------------
    // Location impl
    file.write(b"\n// Auto-generated impl Location\n")?;
    file.write(b"impl Location {\n")?;

    // Location Buy
    file.write(b"\tpub fn get_product_buy(&self) -> HashSet<Product> {\n\t\tmatch self {\n")?;
    for (l, products) in &location_buy {
        file.write(format!("\t\tLocation::{} => HashSet::from_iter(vec!(\n", l).as_bytes())?;
        for (p, _) in products {
            file.write(format!("\t\t\tProduct::{},\n", p).as_bytes())?;
        }
        file.write(format!("\t\t)),\n").as_bytes())?;
    }
    file.write(b"\t\t_ => HashSet::new()}\n")?;    // Handle No Buy Product Location
    file.write(b"\t}\n")?;

    // Location Sell
    file.write(b"\tpub fn get_product_sell(&self) -> HashSet<Product> {\n\t\tmatch self {\n")?;
    for (l, products) in &location_sell {
        file.write(format!("\t\tLocation::{} => HashSet::from_iter(vec!(\n", l).as_bytes())?;
        for (p, _) in products {
            file.write(format!("\t\t\tProduct::{},\n", p).as_bytes())?;
        }
        file.write(format!("\t\t\t)),\n").as_bytes())?;
    }
    file.write(b"\t\t_ => HashSet::new()}\n")?;    // Handle No Sell Product Location
    file.write(b"\t}\n")?;

    // Location Capacity
    file.write(b"\tpub fn get_capacity(&self, product: Product) -> f64 {\n\t\tmatch (self, product) {\n")?;
    for (l, products) in &location_buy {
        for (p, InfoFlux(_, c, ..)) in products {
            file.write(format!("\t\t(Location::{}, Product::{}) => {} as f64,\n", l, p, c).as_bytes())?;
        }
    }
    for (l, products) in &location_sell {
        for (p, InfoFlux(_, c, ..)) in products {
            file.write(format!("\t\t(Location::{}, Product::{}) => {} as f64,\n", l, p, c).as_bytes())?;
        }
    }
    file.write(b"\t\t_ => 0.0}\n")?;   // Handle Wrong Location / Product input
    file.write(b"\t}\n")?;

    // Location Flux
    file.write(b"\tpub fn get_flux(&self, product: Product) -> f64 {\n\t\tmatch (self, product) {\n")?;
    for (l, products) in &location_buy {
        for (p, InfoFlux(.., f, _)) in products {
            file.write(format!("\t\t(Location::{}, Product::{}) => {} as f64,\n", l, p, f).as_bytes())?;
        }
    }
    for (l, products) in &location_sell {
        for (p, InfoFlux(.., f, _)) in products {
            file.write(format!("\t\t(Location::{}, Product::{}) => {} as f64,\n", l, p, f).as_bytes())?;
        }
    }
    file.write(b"\t\t_ => 0.0}\n")?;   // Handle Wrong Location / Product input
    file.write(b"\t}\n")?;

    // Location Destination
    file.write(b"\tpub fn get_destination(&self) -> HashMap<Location, f64> {\n\t\tmatch self {\n")?;
    for (location, destinations) in &location_destination {
        file.write(format!("\t\tLocation::{} => HashMap::from_iter(vec!(\n", location).as_bytes())?;
        for (destination, distance) in destinations {
            file.write(format!("\t\t\t(Location::{}, {} as f64),\n", destination, distance).as_bytes())?;
        }
        file.write(b"\t\t)),\n")?;
    }
    file.write(b"\t}}\n")?;

    // Location impl End
    file.write(b"}\n")?;

    // --------------------------------------------------------------------------------------
    // Enum Product
    file.write(b"\n// Auto-generated Product\n")?;
    file.write(b"#[derive(Clone, Copy, Hash, Eq, PartialOrd, PartialEq, Debug, EnumString)]\n")?;
    file.write(b"pub enum Product {\n")?;
    for (p, ..) in &product_info {
        file.write(format!("\t{},\n", p).as_bytes())?;
    }
    file.write(b"}\n")?;

    // Product Get All
    file.write(b"pub fn get_all_product() -> HashSet<Product> { HashSet::from_iter(vec!(\n")?;
    for (p, ..) in &product_info {
        file.write(format!("\tProduct::{},\n", p).as_bytes())?;
    }
    file.write(b"))}\n")?;

    // --------------------------------------------------------------------------------------
    // Product impl
    file.write(b"\n// Auto-generated impl Product\n")?;
    file.write(b"impl Product {\n")?;

    // Product Capacity Alias
    file.write(b"\tpub fn get_capacity(&self, location: Location) -> f64 { location.get_capacity(*self) }\n")?;

    // Product FLux Alias
    file.write(b"\tpub fn get_flux(&self, location: Location) -> f64 { location.get_flux(*self) }\n")?;

    // Product max
    file.write(b"\tpub fn max(&self) -> f64 {\n\t\tmatch self {\n")?;
    for (p, InfoProd(_,max)) in product_info {
        file.write(format!("\t\t\tProduct::{} => {} as f64,\n", p, max).as_bytes())?;
    }
    file.write(b"\t\t}\n\t}\n")?;

    // Product impl End
    file.write(b"}\n")?;

    // --------------------------------------------------------------------------------------

    // Reconstruct unified market view
    let mut market: HashMap<String, HashMap<String, InfoFlux>> = HashMap::new();
    for (l, products) in &location_buy {
        market.insert(l.clone(), products.clone());
    }
    for (l, products) in &location_sell {
        if market.contains_key(l) {
            for (p, e) in products {
                market.get_mut(l).unwrap().insert(p.clone(), e.clone());
            }
        } else {
            market.insert(l.clone(), products.clone());
        }
    }

    // Get Market
    file.write(b"\n// Auto-generated get_default_market\n")?;
    file.write(b"pub fn get_default_market() -> HashMap<Location, HashMap<Product, (f64, usize)>> { ")?;
    file.write(b"HashMap::from_iter(vec!(\n")?;
    for (l, products) in market {
        file.write(format!("\t(Location::{}, HashMap::from_iter(vec!(\n", l).as_bytes())?;

        for (p, InfoFlux(price, .., stock)) in products {
            file.write(format!("\t\t(Product::{}, ({} as f64, {})),\n", p, price, stock).as_bytes())?;
        }
        file.write(format!("\t))),\n").as_bytes())?;
    }
    file.write(b"))}\n")?;

    Ok(())
}



