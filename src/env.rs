// Maintain a stack of symbol name to value mappings.

use std::collections::HashMap;

pub struct Env<K, V> {
    stack: Vec<HashMap<K, V>>,
}

impl<K, V> Env<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    pub fn new(init: HashMap<K, V>) -> Env<K, V> {
        Env { stack: vec![init] }
    }

    pub fn empty() -> Env<K, V> {
        Env::new(HashMap::new())
    }

    pub fn get(&self, key: K) -> Option<&V> {
        for env in self.stack.iter().rev() {
            if let Some(value) = env.get(&key) {
                return Some(value);
            }
        }
        None
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(env) = self.stack.last_mut() {
            env.insert(key, value);
        }
    }

    pub fn push_env(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop_env(&mut self) -> Option<HashMap<K, V>> {
        self.stack.pop()
    }
}
