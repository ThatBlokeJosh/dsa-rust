// Yo this hashmap is lowkey usable
use std::fmt::Display;
use std::marker::PhantomData;
use std::{ptr::null_mut, usize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Map<K: Hash, T: Sized + Copy> {
    pub bucket: Box<Vec<*mut T>>,
    size: usize,
    a: f64,
    key_type: PhantomData<K>
}

impl<K: Hash, T: Copy + Sized + Display> Map<K, T> {
    pub fn new(size: usize) -> Self {
        let b = vec![null_mut(); size];
        Map {bucket: b.into(), size, a: 0.351876846351345687, key_type: PhantomData}
    } 
    fn calculate_hash(&mut self, t: &K) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    fn hash(&mut self, key: K) -> usize {
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

#[cfg(test)]
mod tests {
    use super::Map;

    #[test]
    fn working_map() {
        let mut m: Map<&str, i32> = Map::new(1000); // The number sets the size of the map, the maximum is a billion
        m.set("foo", 5);
        m.set("oofooo", 9);
        m.set("bar", 4);
        m.print("foo");
        assert_eq!(m.get("foo"), 5);
        assert_eq!(m.get("bar"), 4);
    }

    #[test]
    fn working_generics() {
        let mut m2: Map<i32, i32> = Map::new(1000);
        m2.set(5, 8);
        m2.set(4, 7);
        assert_eq!(m2.get(5), 8);
        assert_eq!(m2.get(4), 7)
    }
}
