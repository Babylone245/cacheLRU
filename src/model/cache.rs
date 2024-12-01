
pub struct Cache<Key, Value> {
    capacity: usize,
    list: Vec<(Key, Value)>
}

impl<Key , Value> Cache<Key, Value>
where
Key:Eq{
    /// Il faut mettre la capacité du cache par exemple -- > Cache::new(3);
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            list: Vec::new(),
        }
    }

    /// Pour stocker des valeurs dans le cache il suffit de donner une clé et la valeur associé à la clé par exemple -->
    ///
    /// cache.put("A", String::from("value_a"));
    /// cache.put("B", String::from("value_b"));
    ///
    /// Représentation :
    /// cache = {
    //     "A" => "value_a",
    //     "B" => "value_b"
    // }
    pub fn put(&mut self, key: Key, value: Value) {
        // Vérifie si la clé existe déjà
        if let Some(pos) = self.list.iter().position(|(k, _)| k == &key) {
            // Supprime la clé existante pour la réinserer en premier
            self.list.remove(pos);
        } else if self.list.len() == self.capacity {
            // Si le cache est plein, supprime le premier élément du coup le moins récent.
            self.list.remove(0);
        }

        // Ajoute la clé et la valeur à la fin et devient la plus recente.
        self.list.push((key, value));
    }

    /// Permet de récupèrer la valeur par en donnant la clé par exemple :
    /// 
    /// cache.put("A", String::from("value_a"));
    /// let my_value = cache.get("A");
    ///
    /// my_value = "value_a";
    pub fn get(&mut self, key: Key) -> Option<&Value> {
        // Trouve la position de la clé si elle existe
        if let Some(pos) = self.list.iter().position(|(k, _)| k == &key) {
            // Déplace la paire (clé, valeur) en fin de liste
            let element = self.list.remove(pos); // Récupère la paire
            self.list.push(element); // Réinsère en tant que "plus récemment utilisé"
            return self.list.last().map(|(_,value)|value);
        }

        None
    }
}