use crate::group::Group;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A GroupTable is simply a mutex-protected hash table,
/// mapping chat group names to actual groups, both
/// managed using reference-counted pointers.
pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> GroupTable {
        GroupTable(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &String) -> Option<Arc<Group>> {
        // since this is sync Mutex
        // we don't need to `await`
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        self.0
            .lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Group::new(name)))
            .clone()
    }
}
