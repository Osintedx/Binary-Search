use std::collections::HashMap;

pub struct Metrics {
    data: HashMap<String, u64>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: HashMap::new(),
        }
    }

    pub fn increment(&mut self, key: &str) {
        let counter = self.data.entry(key.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn get(&self, key: &str) -> Option<&u64> {
        self.data.get(key)
    }
}