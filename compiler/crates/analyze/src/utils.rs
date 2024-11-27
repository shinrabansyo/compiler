use std::collections::HashMap;

#[derive(Debug)]
pub struct LayeredTable<T> {
    elems_map: HashMap<String, HashMap<String, T>>,
}

impl<T> LayeredTable<T> {
    pub(crate) fn new() -> Self {
        LayeredTable {
            elems_map: HashMap::new(),
        }
    }

    pub(crate) fn put(&mut self, namespace: &str, name: &str, value: T) -> anyhow::Result<()> {
        if self.elems_map.get(namespace).is_none() {
            self.elems_map.insert(namespace.to_string(), HashMap::new());
        }
        if self.elems_map.get(namespace).unwrap().get(name).is_some() {
            return Err(anyhow::anyhow!("{} is already defined", name));
        }

        self.elems_map
            .get_mut(namespace)
            .unwrap()
            .insert(name.to_string(), value);

        Ok(())
    }

    pub(crate) fn find_namespace_mut(&mut self, namespace: &str) -> Option<&mut HashMap<String, T>> {
        self.elems_map.get_mut(namespace)
    }

    pub fn find_name(&self, namespace: &str, name: &str) -> Option<&T> {
        if let Some(elems) = self.elems_map.get(namespace) {
            if let Some(elem) = elems.get(name) {
                return Some(elem);
            }
        }

        let dot_pos = namespace.find('.');
        match dot_pos {
            Some(pos) => self.find_name(&namespace[pos+1..], name),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn put() {
        let mut table = LayeredTable::<()>::new();

        assert!(table.put("a", "a", ()).is_ok());
        assert!(table.put("a", "b", ()).is_ok());
        assert!(table.put("a", "b", ()).is_err());

        assert!(table.put("e.a", "a", ()).is_ok());
        assert!(table.put("e.a", "b", ()).is_ok());
        assert!(table.put("e.a", "b", ()).is_err());

        assert!(table.put("g.f.e.a", "a", ()).is_ok());
        assert!(table.put("g.f.e.a", "b", ()).is_ok());
        assert!(table.put("g.f.e.a", "b", ()).is_err());
    }

    #[test]
    fn find_name_rec() {
        let mut table = LayeredTable::<()>::new();

        assert!(table.put("a", "a", ()).is_ok());
        assert!(table.put("f.e.a", "b", ()).is_ok());
        assert!(table.put("h.g.f.e.a", "c", ()).is_ok());

        assert!(table.find_name("f.e.a", "a").is_some());
        assert!(table.find_name("f.e.a", "d").is_none());

        assert!(table.find_name("h.g.f.e.a", "a").is_some());
        assert!(table.find_name("h.g.f.e.a", "d").is_none());
    }
}
