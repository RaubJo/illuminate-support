use serde_json::Value;

pub trait InteractsWithData {
    ///
    /// Get all data from the instance.
    ///
    fn all(&self) -> Value;

    ///
    /// Get data from the instance.
    /// 
    fn data(&self, key: &str, defualt: Value); 

    // ///
    // /// Determine if the data contains a given key.
    // ///
    // fn exists(&self, key: String) -> bool {
    //     self.has(key)
    // }

    // ///
    // /// Determine if the data contains a given key.
    // /// 
    // fn has(&self, key: String) -> bool {
    //     let data = self.all();

    //     for key in data {
    //         if !Self::has_key(&data, key) {
    //             return false;
    //         }
    //     }
    //     true 
    // }

    ///
    /// Determine if the data contains the given keys
    /// 
    fn has_any(&self, keys: &[&str]) -> bool {
        let data = self.all();

        keys.iter().any(|key| Self::has_key(&data, key))
    }

    ///
    // / Execute the closure when the data contains the key 
    /// 
    // fn when_has<T, F>(&self, key: &String, callback: F, default: Option<T>) -> T
    // where
    //     F: FnOnce(&Self, String) -> T,
    //     T: Clone,
    // {
    //     if self.has(key.to_owned()) {
    //         return callback(self, key.to_owned());
    //     }
    
    //     match default {
    //         Some(ref value) => value.clone(),
    //         None => panic!("No value for key and no default provided"),
    //     }
    // }

    ///
    /// Does the data have the given key
    /// 
    /// example: has_key(data, 'user.name')
    /// 
    fn has_key(data: &Value, key: &str) -> bool {
        let mut current = data;
        for part in key.split('.') {
            match current {
                Value::Object(map) => {
                    if let Some(next) = map.get(part) {
                        current = next;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}
