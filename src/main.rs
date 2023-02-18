// ------------------------------------------------------------------------------------------------
/*
/   This file is the entry point of the program, it set few flag from cli options, populate initial
/   node, spawn worker an run A*
*/
// ------------------------------------------------------------------------------------------------

use std::{
    collections::{BinaryHeap, BTreeMap},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
    fs::File,
    cmp::*,
};


use structopt::StructOpt;

mod action;
mod node;
mod data;
mod dataplus;
//mod postprocess;
mod dot;
mod payload;
mod market;
mod state;
mod newnode;

use crate::data::*;
use crate::dot::*;
//use crate::postprocess::*;
use crate::newnode::NewNode;
use uuid::Uuid;


#[derive(Debug, StructOpt)]
#[structopt(name = "Savra, StarCitizen Trade Route Planner", about = "Handy trade route planification with full custom data model, pretty .dot output and some more !\nFull info https://github.com/Bencaddyro/Savra")]
struct Opt {
    #[structopt(short, long, default_value = "100000")]
    /// Starting money
    money: usize,
    #[structopt(short, long, default_value = "564")]
    /// Cargo capacity
    payload: usize,
    #[structopt(short, long, default_value = "Crusader")]
    /// Starting location
    location: Location,
    #[structopt(short, long, default_value = "500")]
    /// Time limit for the run
    time: f64,
    #[structopt(short="n", long, default_value = "1")]
    /// Number of thread for parallel computing
    thread: usize,
}

struct Tag {
  id: Uuid,
  score: f64,
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}
impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}
impl Eq for Tag {}




type NodeHeap = Arc<RwLock<BinaryHeap<Tag>>>;
type NodeStore = Arc<RwLock<BTreeMap<Uuid,Arc<NewNode>>>>;

fn main() {

    // Print state graph
    let out_state = "state.dot";
    let mut file = File::create(out_state).expect(&format!("Unable to open {out_state}"));
    dot_state(&mut file).unwrap();

    //settings
    let opt = Opt::from_args();
    let Opt { money, payload, location, time, thread } = opt;
    
    //init sdd
    let m_heap: NodeHeap = Arc::new(RwLock::new(BinaryHeap::new()));
    let m_store: NodeStore = Arc::new(RwLock::new(BTreeMap::new()));


    let mut handles = Vec::new();
    let root = NewNode::root(money, location, payload);

    m_heap.write().unwrap().push(Tag{id: root.id(), score: root.score()});
    m_store.write().unwrap().insert(root.id(),Arc::new(root));

    {// start thread 0 in advance for queue population
      let m_heap = Arc::clone(&m_heap);
      let m_store = Arc::clone(&m_store);
      let handle = thread::spawn( move || { core_process(0, time, m_heap, m_store) });
      handles.push(handle);
    }

    thread::sleep(Duration::from_millis(1));
    // everyone GET IN HERE
    for n in 1..thread {
        let m_heap = Arc::clone(&m_heap);
        let m_store = Arc::clone(&m_store);
        let handle = thread::spawn(move || { core_process(n, time, m_heap, m_store) });
        handles.push(handle);
    }
    // wait everyone
    for handle in handles {
        handle.join().unwrap();
    }

    // let out_tree = "tree.dot";
    // let mut file = File::create(out_tree).expect(&format!("Unable to open {out_tree}"));
    // dot_tree(&mut file, root.arc_ref).unwrap();


    // Post process
    let Tag{id,score} = m_heap.write().unwrap().pop().unwrap();
    let store = m_store.read().unwrap();
    let node = store.get(&id).unwrap();
    println!("Winner ! (with {score}\n{node}");
    //post_process(output.to_string(), &mut heap).unwrap();


}

fn core_process(n: usize, time_bound: f64, m_heap: NodeHeap, m_store: NodeStore) {
    let mut i = 0;

    loop {
        // get best node
        let Tag{id,score} = m_heap.write().unwrap().pop().unwrap();
        println!("thread {n} | loop {i} | Get node {id}");

        let store = m_store.read().unwrap();
        let node = store.get(&id).unwrap().clone();
        drop(store);
        // drop(store);

        // if stop condition
        if node.time() > time_bound {
            println!("thread {n} | loop {i} | Winner !");
            //put it back in queue
            m_heap.write().unwrap().push( Tag{ id, score });
            break;
        }

        // populate children & add to queue
        let children: Vec<NewNode> = node.gen_children();
        for child in children {
            // println!("thread {n} | Child:\n{child}");
            m_heap.write().unwrap().push( Tag{ id: child.id(), score: child.score() });
            m_store.write().unwrap().insert(child.id(), Arc::new(child));
        }

        // Remove node from store
        m_store.write().unwrap().remove(&id);

        i += 1;
        // Yet another stop condition
        if i > 20 {
            break;
        }
    }
}



