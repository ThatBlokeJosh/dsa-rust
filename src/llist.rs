use std::{fmt::Display, i32, ptr::null_mut};

#[derive(Debug)]
pub struct List<T> {
    pub head: Link<T>,
}

type Link<T> = *mut Node<T>;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

impl<T: Display> List<T> {
    pub fn new() -> Self {
        List { head: null_mut() }
    }

    pub fn push(&mut self, v: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                value: v,
                next: null_mut(),
            }));

            if self.head.is_null() {
                self.head = new_node;
            } else {
                let mut temp = self.head;
                while !(*temp).next.is_null() {
                   temp = (*temp).next; 
                }
                (*temp).next = new_node
            }
        }
    }

    pub fn pop(&mut self) {
        unsafe {
            if self.head.is_null() {
                println!("No elements to pop")
            } else {
                let mut temp = self.head;
                let mut temp2 = self.head;
                while !(*temp2).next.is_null() {
                   temp = temp2; 
                   temp2 = (*temp2).next
                }
                (*temp).next = null_mut()
            }
        }
    }

    pub fn print(&mut self) {
        unsafe {
            if self.head.is_null() {
                println!("No elements to print")
            } else {
                let mut temp = self.head;
                while !(*temp).next.is_null() {
                   println!("{}", (*temp).value);
                   temp = (*temp).next; 
                }
                println!("{}", (*temp).value);
            }
        }
    }
}
