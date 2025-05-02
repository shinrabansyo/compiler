use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Default, Clone)]
pub struct LayeredTable<K, V>
where
    K: Eq + Hash,
{
    tables: VecDeque<HashMap<K, V>>,
}

impl<K, V> LayeredTable<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            tables: VecDeque::from(vec![HashMap::new()]),
        }
    }

    pub fn save(&mut self) {
        self.tables.push_front(HashMap::new());
    }

    pub fn restore(&mut self) {
        self.tables.pop_front();
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.tables.front_mut().unwrap().insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for table in &self.tables {
            if let Some(value) = table.get(key) {
                return Some(value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_layered_table() {
        use super::LayeredTable;

        let mut table = LayeredTable::new();
        table.insert("A", 1);
        assert_eq!(table.get(&"B"), None);
        assert_eq!(table.get(&"A"), Some(&1));

        table.save();

        table.insert("B", 2);
        assert_eq!(table.get(&"B"), Some(&2));
        assert_eq!(table.get(&"A"), Some(&1));

        table.restore();

        assert_eq!(table.get(&"B"), None);
        assert_eq!(table.get(&"A"), Some(&1));
    }
}
