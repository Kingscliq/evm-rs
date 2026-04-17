use primitive_types::U256;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Storage {
    data: HashMap<U256, U256>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Reads a 256-bit value from the storage database. 
    /// According to the EVM specification, reading an uninitialized storage key 
    /// does not error; it naturally returns 0.
    pub fn read(&self, key: U256) -> U256 {
        *self.data.get(&key).unwrap_or(&U256::zero())
    }

    /// Writes a 256-bit value into the storage database at `key`.
    pub fn write(&mut self, key: U256, value: U256) {
        self.data.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_read_write() {
        let mut storage = Storage::new();
        let key = U256::from(100);
        let val = U256::from(999);

        // Read uninitialized key
        assert_eq!(storage.read(key), U256::zero());

        // Write and read back
        storage.write(key, val);
        assert_eq!(storage.read(key), val);

        // Overwrite existing key
        let new_val = U256::from(888);
        storage.write(key, new_val);
        assert_eq!(storage.read(key), new_val);
    }
}
