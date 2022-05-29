
use std::{
    collections::BinaryHeap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};


use structopt::StructOpt;

mod action;
mod node;
mod data;
mod dataplus;
//mod postprocess;
mod cargo;
mod state;

//use crate::node::*;
use crate::data::*;
//use crate::postprocess::*;
use crate::node::Node;


#[derive(Debug, StructOpt)]
#[structopt(name = "Savra, StarCitizen Trade Route Planner", about = "Handy trade route planification with full custom data model, pretty .dot output and some more !\nFull info https://github.com/Bencaddyro/Savra")]
struct Opt {
    #[structopt(short, long, default_value = "100000")]
    /// Starting money
    money: usize,
    #[structopt(short, long, default_value = "564")]
    /// Cargo capacity
    cargo: usize,
    #[structopt(short, long, default_value = "Crusader")]
    /// Starting location
    location: Location,
    #[structopt(short, long, default_value = "500")]
    /// Time limit for the run
    time: f64,
    #[structopt(short="n", long, default_value = "6")]
    /// Number of thread for parallel computing
    thread: usize,
}

fn main() {
    //settings
    let opt = Opt::from_args();
    let Opt { money, cargo, location, time, thread } = opt;
    
    //init sdd
    let m_heap: Arc<Mutex<BinaryHeap<Node>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    //let m_heap = Arc::new(Mutex::new(BinaryHeap::new()));

    let mut handles = Vec::new();
    let root = Node::root(money, location, cargo);
    m_heap.lock().unwrap().push(root.clone());

    {// start thread 0 in advance for queue population
    let m_heap = Arc::clone(&m_heap);
    let handle = thread::spawn(move || { core_process(0, time, m_heap) });
    handles.push(handle);
    }

    thread::sleep(Duration::from_millis(1));

    // everyone GET IN HERE
    for n in 1..thread {
        let m_heap = Arc::clone(&m_heap);
        let handle = thread::spawn(move || { core_process(n, time, m_heap) });
        handles.push(handle);
    }
    // wait everyone

    for handle in handles {
        handle.join().unwrap();
    }

    /*
    // Post process
    let mut heap = m_heap.lock().unwrap();
    let output = "../";
    //post_process(output.to_string(), &mut heap).unwrap();

    */

}

fn core_process(n: usize, time_bound: f64, m_heap: Arc<Mutex<BinaryHeap<Node>>>) {
    let mut i = 0;
    loop {
        // get best node
        let mut heap = m_heap.lock().unwrap();
        let node: Node = heap.pop().unwrap();
        drop(heap);
        
        println!("thread {}, loop {} \n{}",n,i,*node);

        // if stop condition
        //println!("Test stop condition");
        if node.time() > time_bound{
            println!("thread {}, loop {} winner",n, i);
            //put it back in queue
            let mut heap = m_heap.lock().unwrap();
            heap.push(node);

            drop(heap);
            break;
        }

        // populate children & add to queue
        //println!("Gen children");
        let children: Vec<Node> = node.gen_children();//gen_children(Arc::clone(&node));
        for child in children {
            //println!("thread {}\n{}",n,child);
            child.update_score(time_bound);
            let mut heap = m_heap.lock().unwrap();
            heap.push(child);
            drop(heap);
        }
        i += 1;
        //println!("Loop over");
    }

}



