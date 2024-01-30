use std::fmt::Display;
use std::marker::PhantomData;
use std::{ptr::null_mut, usize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Map<K: Hash, T: Sized + Copy> {
    pub bucket: Vec<*mut T>,
    size: usize,
    a: f64,
    key_type: PhantomData<K>
}

impl<K: Hash, T: Copy + Sized + Display> Map<K, T> {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, null_mut());
        Map {bucket: v, size, a: 0.351876846351345687, key_type: PhantomData}
    } 
    pub fn calculate_hash(&mut self, t: &K) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    pub fn hash(&mut self, key: K) -> usize {
        let mut u = self.calculate_hash(&key);
        u = u / 1000000;
        let mut out = u as f64 * self.a;
        out = out % 1.0;
        out = out * self.size as f64;
        return out.round() as usize;
    }
    pub fn set(&mut self, key: K, value: T) {
        let hash = self.hash(key);
        let v = Box::into_raw(Box::new(value));
        self.bucket[hash] = v
    }
    pub fn get(&mut self, key: K) -> T {
        unsafe {
            let hash = self.hash(key);
            return *self.bucket[hash];
        }
    }
    pub fn print(&mut self, key: K) {
        println!("{}", self.get(key));
    }
}
