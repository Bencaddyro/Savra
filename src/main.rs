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
};

use uuid::Uuid;
use structopt::StructOpt;

mod data;
mod dot;
mod payload;
mod market;
mod newnode;
mod utils;

use crate::utils::*;
use crate::data::*;
use crate::dot::*;
use crate::newnode::*;


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
    #[structopt(short, long, default_value = "350")]
    /// Time limit for the run
    time: f64,
    #[structopt(short="n", long, default_value = "6")]
    /// Number of thread for parallel computing
    thread: usize,
    #[structopt(long)]
    /// Do not run solver but export data as .dot file
    map: bool,
}

type NodeHeap = Arc<RwLock<BinaryHeap<Tag>>>;
type NodeStore = Arc<RwLock<BTreeMap<Uuid,Arc<NewNode>>>>;

fn main() {
  // Get CLI options
  let Opt { money, payload, location, time, thread, map } = Opt::from_args();

  if map { // Export data to .dot file
    let out_state = "state.dot";
    let mut file = File::create(out_state).expect(&format!("Unable to open {out_state}"));
    dot_universe(&mut file).unwrap();
    return;
  }

  // Initialization NodeHeap, NodeStore, Workers
  let m_heap: NodeHeap = Arc::new(RwLock::new(BinaryHeap::new()));
  let m_store: NodeStore = Arc::new(RwLock::new(BTreeMap::new()));
  let mut handles = Vec::new();

  // Create root node
  let root = NewNode::root(money, location, payload);
  m_heap.write().unwrap().push( Tag{ id: root.id(), score: root.score() });
  m_store.write().unwrap().insert(root.id(),Arc::new(root));

  {// Start worker 0 in advance for queue population
    let m_heap = Arc::clone(&m_heap);
    let m_store = Arc::clone(&m_store);
    let handle = thread::spawn( move || { worker_job( 0, time, m_heap, m_store) });
    handles.push(handle);
  }
  thread::sleep(Duration::from_millis(1));

  // Everyone GET IN HERE
  for n in 1..thread {
    let m_heap = Arc::clone(&m_heap);
    let m_store = Arc::clone(&m_store);
    let handle = thread::spawn(move || { worker_job( n, time, m_heap, m_store) });
    handles.push(handle);
  }

  // Wait for all workers
  for handle in handles {
      handle.join().unwrap();
  }

  // Retrieve n top solutions
  for i in 0..5 {
    let Tag{ id, score} = m_heap.write().unwrap().pop().unwrap();
    let store = m_store.read().unwrap();
    let node = store.get(&id).unwrap();
    println!("Winner {i} {score}\n{node}");
    format!("{}",dot_node(node));
  }
}

fn worker_job(n: usize, time_limit: f64, m_heap: NodeHeap, m_store: NodeStore) {
  let mut i = 0;
  let max_step = -1;
  loop {
    // Get best node
    let Tag{ id, score} = m_heap.write().unwrap().pop().expect("Oops ! Heap is empty !");
    // println!("thread {n:<2} | loop {i:<10} | Get node {id}");

    let store = m_store.read().unwrap();
    let len = store.len();
    let node = store.get(&id).expect("Oops ! Cannot find {id} in NodeStore !").clone();
    println!("thread {n:<2} | loop {i:<10} | Total Store Size {len} * {} = {}", std::mem::size_of::<NewNode>(), len * std::mem::size_of::<NewNode>());
    drop(store);

    // If stop condition
    if node.time() >= time_limit || i == max_step {
      // println!("thread {n:<2} | loop {i:<10} | Winner : {id} = {score}");
      m_heap.write().unwrap().push( Tag{ id, score });
      break;
    }

    // Else populate children & add to queue
    let children = node.gen_children(time_limit);
    for child in children {
      // println!("thread {n} | Child:\n{child}");
      m_store.write().unwrap().insert(child.id(), child.clone());
      m_heap.write().unwrap().push( Tag{ id: child.id(), score: child.score() });
    }

    // Remove node from store
    m_store.write().unwrap().remove(&id);
    i += 1;
  }
}



