use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::{ptr::null_mut, usize};

use crate::hmap::{Map};
use crate::llist::List;

#[derive(Debug, Clone)]
pub struct Vertex<T: Copy + Hash + Clone + Debug + Display> {
    pub value: T,
    pub index: usize,
    pub visited: bool,
    pub edges: Map<usize, usize>
}

#[derive(Debug)]
pub struct Graph<T: Hash + Copy + Debug + Display> {
    pub nodes: Vec<Vertex<T>>
}

impl <T: Hash + Copy + Debug + Display> Graph<T> {
    pub fn new() -> Self {
        Graph {nodes: vec![]}
    }

    pub fn add(&mut self, value: T) {
        let vertex = Vertex { value, index: self.nodes.len(), edges: Map::new(1000), visited: false }; 
        self.nodes.push(vertex)
    }

    pub fn edit(&mut self, index: usize, value: T) {
        self.nodes[index].value = value
    } 

    pub fn add_edge(&mut self, i1: usize, i2: usize) {
        self.nodes[i1].edges.set(i2, i2);
        self.nodes[i2].edges.set(i1, i1);
    }

    pub fn reset(&mut self) {
        for i in 0..self.nodes.len() {
            self.nodes[i].visited = false 
        } 
    }

    fn inner_dfs(&mut self, index: usize) {
        self.nodes[index].visited = true;

        for i in self.nodes[index].edges.keys.clone() {
            if !self.nodes[i].visited {
                println!("{:?}", self.nodes[i].value);
                self.inner_dfs(i);
            }
        }
    }

    pub fn dfs(&mut self, index: usize) {
        self.inner_dfs(index);
        self.reset();
    }
    
    pub fn bfs(&mut self, index: usize) {
        let mut queue: List<usize> = List::new();
        self.nodes[index].visited = true;
        queue.push(self.nodes[index].index);
        unsafe {
            while !queue.empty() {
                let curr = &self.nodes[(*queue.head).value]; 
                queue.unshift();
                // println!("{:?}", curr.value);
                for i in curr.edges.keys.clone() {
                    if !self.nodes[i].visited {
                        self.nodes[i].visited = true;
                        queue.push(self.nodes[i].index);
                    }
                }
            }
        }
        self.reset()
    }
}
