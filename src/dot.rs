use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use crate::data::*;
use crate::node::*;

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
            // TODO Handle bidirection with same time
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


pub fn dot_tree(file: &mut File, node: Arc<NodeData>) -> std::io::Result<()>{
    file.write(b"digraph tree {\n\trankdir=\"LR\";\n\toverlap=false;\n")?;
    dot_tree_rec(file, node).unwrap();
    file.write(b"}")?;
    Ok(())
}

fn dot_tree_rec(file: &mut File, node: Arc<NodeData>) -> std::io::Result<()>{
    file.write(node.to_dot().as_bytes())?;
    for child in node.get_children() {
        dot_tree_rec(file, child).unwrap();
    }
    Ok(())
}
