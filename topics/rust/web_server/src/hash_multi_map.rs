use std::collections::HashMap;
use std::hash::Hash;

pub struct HashMultiMap<Key, Value> {
    map: HashMap<Key, Vec<Value>>
}

impl<Key, Value> HashMultiMap<Key, Value> 
    where Key : Hash + Eq + Clone {
    pub fn new() -> HashMultiMap<Key, Value> {
        HashMultiMap {
            map: HashMap::new()
        }
    }

    pub fn push_to_key(&mut self, key: &Key, value: Value) {
        let existing_key = self.map.get_mut(key);

        if existing_key.is_some() {
            let existing_list = existing_key.unwrap();
            existing_list.push(value)
        } else {
            let new_list = vec!(value);
            self.map.insert(key.clone(), new_list);
        }
    }

    pub fn get(&self, key: &Key) -> Option<&Vec<Value>> {
        self.map.get(key)
    }
}
