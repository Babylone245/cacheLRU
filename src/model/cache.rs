use std::collections::{HashMap, VecDeque};

pub struct Cache<Key, Value> {
    capacity: usize,
    index: VecDeque<Key>,
    cache: HashMap<Key, Value>,
}

impl<Key, Value> Cache<Key, Value>
where
    Key: Eq + std::hash::Hash + Clone {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            index: VecDeque::new(),
            cache: HashMap::new(),
        }
    }

    /// Pour stocker des valeurs dans le cache il suffit de donner une clé et la valeur associé à la clé par exemple -->
    ///
    /// cache.put("A", String::from("value_a"));
    /// cache.put("B", String::from("value_b"));
    ///
    /// Représentation :
    /// cache = {
    ///    "A" => "value_a",
    ///    "B" => "value_b"

    ///
    /// ```
    ///
    /// use cache::model::Cache;
    ///
    ///
    /// let mut cache = Cache::new(3);
    /// cache.put("A", String::from("value_a"));
    /// assert_eq!(cache.get("A"), Some(&String::from("value_a")));
    /// ```
    pub fn put(&mut self, key: Key, value: Value) {
        if self.cache.contains_key(&key) {
            // Conserve les clés qui sont différents que celle qu'on get (la supprime de la liste)
            self.index.retain(|k| k != &key);
        } else if self.cache.len() == self.capacity {
            if let Some(old_key) = self.index.pop_front() {
                self.cache.remove(&old_key);
            }
        }
        self.index.push_back(key.clone());
        self.cache.insert(key, value);
    }

    /// Permet de récupèrer la valeur par en donnant la clé par exemple :
    ///
    /// cache.put("A", String::from("value_a"));
    /// let my_value = cache.get("A");
    ///
    /// my_value = "value_a";
    ///
    /// ```
    ///
    /// use cache::model::Cache;
    ///
    ///
    /// let mut cache = Cache::new(3);
    /// cache.put("A", String::from("value_a"));
    /// let mut cache = cache.get("A");
    /// assert_eq!(cache, Some(&String::from("value_a")));
    /// ```
    pub fn get(&mut self, key: Key) -> Option<&Value> {
        if let Some(value) = self.cache.get(&key) {
            // Conserve les clés qui sont différents que celle qu'on get (la supprime de la liste)
            self.index.retain(|k| k != &key);
            // Et après on la remet à la fin de la liste pour qu'elle soit la plus récente ajouté.
            self.index.push_back(key.clone());
            return Some(value);
        }
        None
    }
}