use std::i32;

use graph::Graph;

mod llist;
mod graph;
mod bst;
mod hmap;
mod bs;

fn main() {
    let mut g: Graph<i32> = Graph::new();
    g.add(5);
    g.add(8);
    g.add_edge(0, 1);
    g.add(7);
    g.add_edge(1, 2);
    for v in &g.nodes {
        print!("{:?}", v.value);
        print!(" - ");
        print!("{:?}", v.edges.keys);
        println!();
    }
}
