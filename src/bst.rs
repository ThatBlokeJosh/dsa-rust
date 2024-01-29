// Binary search tree implementation

use std::ptr::null_mut;
use std::fmt::Display;

type Link<T> = *mut Node<T>;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T, 
    pub left: Link<T>,
    pub right: Link<T>,
}

impl<T:Display + PartialOrd> Node<T> {
    pub fn new(v: T) -> Self {
        Node {value: v, left: null_mut(), right: null_mut()}
    }

    pub fn insert(&mut self, v: T) {
        unsafe {
            let new_node =  Box::into_raw(Box::new(Node {
                value: v,
                left: null_mut(),
                right: null_mut(),
            }));

            let mut root = self;
            
            loop {
                if (*root).value > (*new_node).value {
                    if (*root).left.is_null() {
                        (*root).left = new_node;
                        break;
                    } else {
                        root = &mut (*(*root).left);
                    }
                } else if (*root).value < (*new_node).value {
                    if (*root).right.is_null() {
                        (*root).right = new_node;
                        break;
                    } else {
                        root = &mut (*(*root).right);
                    }
                } else {
                    let message = format!("Value {} is already in the tree", (*new_node).value);
                    println!("{}", message);
                    break;
                }
            }
        }
    }

    pub fn search(&mut self, v: T) -> bool {
        unsafe {
            if (*self).value == v {
                return true;
            } else if (*self).value > v && !(*self).left.is_null() {
                return (*(*self).left).search(v);
            } else if (*self).value < v && !(*self).right.is_null() {
                return (*(*self).right).search(v);
            } else {
                return false
            }
        }
    }

    pub fn all(&mut self) {
        println!("{}", self.value);

        unsafe {
            if !self.right.is_null() {
                (*self.right).all() 
            } 
            if !self.left.is_null() {
                (*self.left).all() 
            }
        }
    }
}
