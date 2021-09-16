use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod action;
mod node;
mod data;
mod dataplus;
mod postprocess;

use crate::node::*;
use crate::data::*;
use crate::postprocess::*;

fn main() {
    //settings
    let wallet = 100_000;
    let cargo = 64;
    let start_location = Location::Crusader;
    let time_bound = 500.0;
    
    let n_thread = 6;
    
    //init sdd
    let m_heap = Arc::new(Mutex::new(BinaryHeap::new()));
    let mut handles = vec![];
    
    //init root
    let root = Arc::new(root(wallet, start_location, cargo));
    m_heap.lock().unwrap().push(root);
    
    {// start thread 0 in advance for queue population
    let m_heap = Arc::clone(&m_heap);
    let handle = thread::spawn(move || { core_process(0, time_bound, m_heap) });
    handles.push(handle);
    }
    thread::sleep(Duration::from_millis(1));
    
    // everyone GET IN HERE
    for n in 1..n_thread {
        let m_heap = Arc::clone(&m_heap);
        let handle = thread::spawn(move || { core_process(n, time_bound, m_heap) });
        handles.push(handle);
    }
    
    // wait everyone
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Post process
    let mut heap = m_heap.lock().unwrap();
    let output = "../";
    post_process(output.to_string(), &mut heap).unwrap();
}

fn core_process(n: usize, time_bound: f64, m_heap: Arc<Mutex<BinaryHeap<Arc<Node>>>>) {
    let mut i = 0;
    loop {
        println!("thread {}, loop {}",n, i);
        
        // get best node
        let mut heap = m_heap.lock().unwrap();
        let node: Arc<Node> = heap.pop().unwrap();      
        drop(heap);
        
        // if stop condition
        if node.time > time_bound{
            println!("thread {}, loop {} winner",n, i);
            //put it back in queue
            let mut heap = m_heap.lock().unwrap();
            heap.push(Arc::clone(&node));
            drop(heap);
            break;
        }
        
        // populate children & add to queue
        let children = gen_children(Arc::clone(&node), time_bound);
        for child in children {
            //println!("thread {}\n{}",n,child);
            let mut heap = m_heap.lock().unwrap();
            heap.push(child);
            drop(heap);
        }
        i += 1;
    }
}



