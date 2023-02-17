// ------------------------------------------------------------------------------------------------
/*
/   This file define post process function to display an create various chart displaying data model
*/
// ------------------------------------------------------------------------------------------------

/*
use std::io::Write;
use std::fs::File;
use std::collections::BinaryHeap;
use std::sync::Arc;
use chrono;
use crate::data::*;
use crate::node::*;

fn write_state(file: &mut File) -> std::io::Result<()> {

    // Data
    let map = get_map();
    let buy = get_buy();
    let sell = get_sell();
    
    // Subgraph header
    file.write(b"subgraph {\n")?;
    
    for (location,entry) in map {
        // Location graph
        for (destination,_value) in entry {
            //file.write(format!("{} -> {} [label=\"{}\"];\n", location, destination, _value).as_bytes()).unwrap();
            file.write(format!("\"{}\" -> \"{}\" ;\n", location, destination).as_bytes())?;
        }
        // Price table
        if buy.contains_key(&location) | sell.contains_key(&location) {
            file.write(&format!(" \"T{}\" [shape=Mrecord,label=\"",location).as_bytes())?;
            if buy.contains_key(&location) {
                file.write(b"BUY ")?;
                for (product, price) in buy.get(&location).unwrap() {
                    file.write(&format!("| {{ {} | {:2}¤UEC }} ", product, price).as_bytes())?;
                }
                file.write(b"| ")?;
            }
            if buy.contains_key(&location) {
                file.write(b"SELL ")?;
                for (product, price) in sell.get(&location).unwrap() {
                    file.write(&format!("| {{ {} | {:2}¤UEC }} ", product, price).as_bytes())?;
                }
            }
            file.write(&format!("\"];\n\"{0}\" -> \"T{0}\" [arrowhead=none];\n",location).as_bytes())?;
        }  
    }
    // Subgraph footer
    file.write(b"}\n")?;
    Ok(())
}

fn write_price(file: &mut File) -> std::io::Result<()> {
    // Subgraph header
    file.write(b"subgraph {\n")?;
    file.write(b"price [shape=Mrecord,label=\"{ Product | min BUY | max Sell | Best }")?;
    
    for p in Product::all() {
        if (p.max() != 0.0) & (p.min() != 0.0) {
            file.write(format!(" | {{ {} | {} | {} | {:.2} }}",p,p.min(),p.max(),p.max()-p.min()).as_bytes())?;
        }
    }
    // Subgraph footer
    file.write(b"\"];\n")?;
    file.write(b"}\n")?;
    Ok(())
}

fn write_dot(file: &mut File, heap: &mut BinaryHeap<Arc<Node>>) -> std::io::Result<()> {    
    // Backtrace Winner
    file.write(b"node [color=\"red\"];\nedge [color=\"red\"];\n")?;
    let mut node = heap.pop().unwrap();
    while !node.is_root() {
        file.write(node.dot().as_bytes())?;
        node = node.parent();
    }
    file.write(node.dot().as_bytes())?;
    
    // Leaf
    /*
    file.write(b"node [color=\"blue\"];edge [color=\"black\"];\n")?;
    while !heap.is_empty() {
        node = tree.get(&heap.pop().unwrap().1).unwrap();
        file.write(node.dot().as_bytes())?;
        draw.insert(node.id);
    }
    */    
    // Other Node
    /*
    file.write(b"node [color=\"black\"];\nedge [color=\"black\"];\n")?;
    for (id,n) in tree {
        if !draw.contains(id) {
            file.write(n.dot().as_bytes())?;
        }
    }
    */
    Ok(())
}

pub fn post_process(path: String, heap: &mut BinaryHeap<Arc<Node>>) -> std::io::Result<()>{

    let mut file = File::create(path.clone()+"state.dot")?;    
    file.write(b"digraph tree {\nrankdir=\"LR\";\noverlap=false;\n")?;
    write_state(&mut file)?;
    write_price(&mut file)?;
    file.write(b"}")?;
    
    let node = heap.pop().unwrap();
    println!("Winner !\n{}",node);
    backtrace(&node);
    heap.push(node);
    
    let mut file = File::create(path.clone()+"graph.dot")?;    
    file.write(b"digraph tree {\nrankdir=\"LR\";\noverlap=false;\n")?;
    write_dot(&mut file, heap)?;
    file.write(b"}")?;

    Ok(())
}

fn backtrace(leaf: &Arc<Node>) {

    let mut file = File::create(format!("../rez{:?}",chrono::offset::Utc::now())).unwrap();
    file.write(format!("Winner !\n{}\n",leaf).as_bytes());
    

    let mut actions = Vec::new();
    let mut node: Arc<Node> = Arc::clone(leaf);
    while !node.is_root() {
        actions.push(format!("[{};{}¤UEC] : {}", node.location(), node.wallet(), node.action()));
        let next = node.parent();
        node = next;
    }
    

    
    actions.reverse();
    println!("Trace :");
    for action in actions {
        println!("{}",action);
        file.write(action.as_bytes());
        file.write(b"\n");
    }

}
*/
