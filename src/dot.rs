// ------------------------------------------------------------------------------------------------
/*
/   This file handle post processing of data stucture to dot file
*/
// ------------------------------------------------------------------------------------------------

use std::io::Write;
use std::fs::File;
use std::sync::Arc;
use crate::data::*;
use crate::node::*;

pub fn dot_state(file: &mut File) -> std::io::Result<()> {

    // Subgraph header
    file.write(b"digraph {\nrankdir=\"LR\";\noverlap=false;\n")?;

    for location in get_all_location() {
        // Location graph
        for (destination, ..) in location.get_destination() {
            // TODO Handle bidirection with same time
            file.write(format!("\"{location}\" -> \"{destination}\" ;\n").as_bytes())?;
        }
        // Price table
        if !location.get_product_buy().is_empty() | !location.get_product_sell().is_empty() {
            file.write(&format!("\t\"T{location}\" [shape=Mrecord, label=\"\n").as_bytes())?;
            if !location.get_product_buy().is_empty() {
                file.write(b"\tBUY\n")?;
                for product in location.get_product_buy() {
                    let price = 5.0; // TODO dynmaic price
                    file.write(format!("\t| {{ {product} | {price:2} ¤UEC }}\n").as_bytes())?;
                }
                file.write(b"| ")?;
            }
            if !location.get_product_sell().is_empty() {
                file.write(b"\tSELL\n")?;
                for product in location.get_product_sell() {
                    let price = 5.0; // TODO dynmaic price
                    file.write(&format!("\t| {{ \"{product}\" | {price:2} ¤UEC }}\n").as_bytes())?;
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
    file.write(b"digraph tree {\n\trankdir=\"LR\";\n\toverlap=scale;\n\tranksep=0.02\n")?;
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
