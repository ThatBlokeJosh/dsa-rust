use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::{ptr::null_mut, usize};

use crate::hmap::{Map};

#[derive(Debug, Clone)]
pub struct Vertex<T: Copy + Hash + Clone> {
    pub value: T,
    pub index: usize,
    pub edges: Map<T, usize>
}

#[derive(Debug)]
pub struct Graph<T: Hash + Copy> {
    pub nodes: Vec<Vertex<T>>
}

impl <T: Hash + Copy> Graph<T> {
    pub fn new() -> Self {
        Graph {nodes: vec![]}
    }

    pub fn add(&mut self, value: T) {
        let vertex = Vertex { value, index: self.nodes.len(), edges: Map::new(1000) }; 
        self.nodes.push(vertex)
    }

    pub fn edit(&mut self, index: usize, value: T) {
        self.nodes[index].value = value
    } 

    pub fn add_edge(&mut self, i1: usize, i2: usize) {
        let v1 = self.nodes[i1].value;
        let v2 = self.nodes[i2].value;
        
        self.nodes[i1].edges.set(v2, i2);
        self.nodes[i2].edges.set(v1, i1);
    }
}
