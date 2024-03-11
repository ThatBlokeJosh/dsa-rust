use std::i32;

use graph::Graph;

mod llist;
mod graph;
mod bst;
mod hmap;
mod bs;

fn main() {
    let mut g: Graph<i32> = Graph::new();
    g.add(0);
    g.add(1);
    g.add_edge(0, 1);
    g.add(2);
    g.add_edge(0, 2);
    g.add(3);
    g.add(4);
    g.add_edge(1, 3);
    g.add_edge(2, 4);
    for v in &g.nodes {
        print!("{:?}", v.value);
        print!(" - ");
        print!("{:?}", v.edges.keys);
        println!();
    }
    g.dfs(0)
}
