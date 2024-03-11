// Yo this hashmap is lowkey usable
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::{ptr::null_mut, usize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Map<K: Hash + Copy, T: Sized + Copy> {
    pub bucket: Box<Vec<Value<T>>>,
    pub keys: Vec<K>, 
    size: usize,
    a: f64,
    key_type: PhantomData<K>
}

#[derive(Debug, Clone)]
pub struct Value<T: Sized + Copy> {
    pub value: *mut T,
    pub index: usize
}

impl<K: Hash + Copy, T: Copy + Sized + Display + Debug> Map<K, T> {
    pub fn new(size: usize) -> Self {
        let b = vec![Value{value: null_mut(), index: 0}; size];
        let k = Vec::new();
        Map {bucket: b.into(), size, a: 0.351876846351345687, key_type: PhantomData, keys: k}
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
        self.keys.push(key);
        self.bucket[hash] = Value{value: v, index: self.keys.len() - 1};
    }

    pub fn get(&mut self, key: K) -> Option<T> {
        unsafe {
            let hash = self.hash(key);
            if self.bucket[hash].value.is_null() {
                return None;
            }
            return Some(*self.bucket[hash].value);
        }
    }

    pub fn print(&mut self, key: K) {
        println!("{:?}", self.get(key));
    }

    pub fn contains(&mut self, key: K) -> bool {
        return !self.get(key).is_none();
    }

    pub fn delete(&mut self, key: K) {
        let hash = self.hash(key);
        self.keys.remove(self.bucket[hash].index);
        self.bucket[hash] = Value{value: null_mut(), index: 0}
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
        assert_eq!(m.get("foo"), Some(5));
        assert_eq!(m.get("bar"), Some(4));
        m.set("foo", 3);
        assert_eq!(m.get("foo"), Some(3));
        assert_eq!(m.contains("foo"), true);
        assert_eq!(m.contains("hello"), false);
        m.delete("foo");
        assert_eq!(m.get("foo"), None);
    }
}
