// Custom hash map with horible hashing function
use std::{ptr::null_mut, usize};

#[derive(Debug)]
pub struct Map<T: Sized> {
    pub bucket: Vec<*mut T>,
    size: usize,
    a: f64,
}

impl<T: Copy> Map<T> {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, null_mut());
        Map {bucket: v, size, a: 0.35784465456464643645288795}
    } 
    pub fn hash(&mut self, key: &str) -> usize {
        let b = key.bytes();

        let mut t = "".to_string();

        for c in b {
           t.push_str(&c.to_string()) 
        }

        let u: u64 = t.parse().unwrap();
        let mut out = u as f64 * self.a;
        out = out % 1.0;
        out = out * self.size as f64;
        return out.round() as usize;
    }
    pub fn set(&mut self, key: &str, value: T) {
        let hash = self.hash(key);
        let v = Box::into_raw(Box::new(value));
        self.bucket[hash] = v
    }
    pub fn get(&mut self, key: &str) -> T {
        unsafe {
            let hash = self.hash(key);
            return *self.bucket[hash];
        }
    }
}
