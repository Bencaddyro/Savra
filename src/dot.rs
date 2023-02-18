// ------------------------------------------------------------------------------------------------
/*
/   This file handle post processing of data stucture to dot file
*/
// ------------------------------------------------------------------------------------------------

use std::io::Write;
use std::fs::File;
use crate::data::*;
use crate::RefNewNode;
use crate::Action::{Travel,Buy,Sell,Wait};


pub fn dot_universe(file: &mut File) -> std::io::Result<()> {

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
                    let price = 5.0; // TODO dynamic price
                    file.write(format!("\t| {{ {product} | {price:2} ¤UEC }}\n").as_bytes())?;
                }
                file.write(b"| ")?;
            }
            if !location.get_product_sell().is_empty() {
                file.write(b"\tSELL\n")?;
                for product in location.get_product_sell() {
                    let price = 5.0; // TODO dynamic price
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

pub fn dot_node(node: &RefNewNode) -> String {
  let mut s = String::new();


  let p_id = node.parent().simple();
  let id = node.id().simple();
  let wallet = node.wallet();
  let time = node.time();
  let location = node.location();
  let score = node.score();
  let total = node.payload().capacity;
  let current = total - node.payload().empty();

  s.push_str(&format!("\t\"{id}\" [shape=record, label=\" {{ {wallet} ¤UEC | t={time} }} | {{ {location} | s={score:.3} }} | CARGO {current}/{total}"));

  // for (product, amount) in node.payload().payload {
    // s.push_str(&format!(" | {{ {product} | {amount} }}"));
  // }

  s.push_str(&format!("\"];\n"));
  let action = node.actions()[0];
  match action {
    Travel{location, duration, distance} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ MOVE | {duration}s }} | {{ {location} | {distance} }}\"];\n")),
    Buy{product, amount, price} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ BUY | {product} }} | {{ {amount} aSCU | {price} ¤UEC }}\"];\n")),
    Sell{product, amount, price} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"{{ SELL | {product} }} | {{ {amount} aSCU | {price} ¤UEC }}\"];\n")),
    Wait{duration} => s.push_str(&format!("\"A{id}\" [shape=Mrecord, label=\"WAIT | {duration}s\"];\n")),
  }
  s.push_str(&format!("\t\"{p_id}\" -> \"A{id}\";\n"));
  s.push_str(&format!("\t\"A{id}\" -> \"{id}\";\n"));

  s
}
