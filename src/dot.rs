use std::io::Write;
use std::fs::File;
use crate::data::*;

pub fn dot_state(file: &mut File) -> std::io::Result<()> {

    // Data
    let map = get_map();
    let buy = get_buy();
    let sell = get_sell();

    // Subgraph header
    file.write(b"digraph {\nrankdir=\"LR\";\noverlap=false;\n")?;

    for (location, entry) in map {
        // Location graph
        for (destination, ..) in entry {
            //file.write(format!("{} -> {} [label=\"{}\"];\n", location, destination, _value).as_bytes()).unwrap();
            file.write(format!("\"{location}\" -> \"{destination}\" ;\n").as_bytes())?;
        }
        // Price table
        if buy.contains_key(&location) | sell.contains_key(&location) {
            file.write(&format!("\t\"T{location}\" [shape=Mrecord, label=\"\n").as_bytes())?;
            if buy.contains_key(&location) {
                file.write(b"\tBUY\n")?;
                for (product, price) in buy.get(&location).unwrap() {
                    file.write(format!("\t| {{ {product} | {price:2} ¤UEC }}\n").as_bytes())?;
                }
                file.write(b"| ")?;
            }
            if buy.contains_key(&location) {
                file.write(b"\tSELL\n")?;
                for (product, price) in sell.get(&location).unwrap() {
                    file.write(&format!("\t| {{ {product} | {price:2} ¤UEC }}\n").as_bytes())?;
                }
            }
            file.write(format!("\"];\n\"{location}\" -> \"T{location}\" [arrowhead=none];\n").as_bytes())?;
        }
    }
    // Subgraph footer
    file.write(b"}\n")?;
    Ok(())
}
