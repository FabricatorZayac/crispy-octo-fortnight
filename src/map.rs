use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

pub struct HashMap<K, V> {
    inner: Vec<ListMap<K, V>>,
}

#[derive(Clone)]
pub struct ListMap<K, V> {
    inner: Vec<(K, V)>,
}

pub trait Map<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get_ref(&self, key: K) -> Option<&V>;
    fn get_mut_ref(&mut self, key: K) -> Option<&mut V>;
    fn get(&self, key: K) -> Option<V>
        where V: Clone;
    fn contains_key(&self, key: &K) -> bool;
}

// No automatic resizing for now
impl<K, V> Map<K, V> for HashMap<K, V>
    where K: Hash + PartialEq + Clone,
          V: Clone {
    fn put(&mut self, key: K, value: V) {
        let idx = self.get_idx(&key);
        self.inner[idx].put(key, value);
    }

    fn get_ref(&self, key: K) -> Option<&V> {
        self.inner[self.get_idx(&key)].get_ref(key).map(|value| value)
    }

    fn get_mut_ref(&mut self, key: K) -> Option<&mut V> {
        let idx = self.get_idx(&key);
        self.inner[idx].get_mut_ref(key).map(|value| value)
    }

    fn get(&self, key: K) -> Option<V>
        where V: Clone {
        self.inner[self.get_idx(&key)].get(key).map(|value| value)
    }

    fn contains_key(&self, key: &K) -> bool {
        self.inner[self.get_idx(&key)].contains_key(key)
    }
}

impl<K, V> Map<K, V> for ListMap<K, V>
    where K: PartialEq {
    fn put(&mut self, key: K, value: V) {
        if self.contains_key(&key) {
            *self.get_mut_ref(key).unwrap() = value;
            return;
        }
        self.inner.push((key, value));
    }

    fn get_ref(&self, key: K) -> Option<&V> {
        for i in self.inner.iter() {
            if i.0 == key {
                return Some(&i.1);
            }
        }
        None
    }

    fn get_mut_ref(&mut self, key: K) -> Option<&mut V> {
        for i in self.inner.iter_mut() {
            if i.0 == key {
                return Some(&mut i.1);
            }
        }
        None
    }

    fn get(&self, key: K) -> Option<V>
        where V: Clone {
        for i in self.inner.iter() {
            if i.0 == key {
                return Some(i.1.clone());
            }
        }
        None
    }

    fn contains_key(&self, key: &K) -> bool {
        for i in self.inner.iter() {
            if i.0 == *key {
                return true;
            }
        }
        false
    }
}

impl<K, V> HashMap<K, V>
    where K: Hash + Clone,
          V: Clone {
    pub fn new() -> Self {
        HashMap { inner: vec![ListMap::<K, V>::new(); 16] }
    }

    fn get_idx(&self, key: &K) -> usize{
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.inner.len() as usize
    }
}

impl<K, V> ListMap<K, V>
    where K: Clone,
          V: Clone {
    pub fn new() -> Self {
        ListMap { inner: Vec::new() }
    }
}
