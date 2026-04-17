use primitive_types::U256;

#[derive(Debug, Clone, Default)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Read exactly 32 bytes (a 256-bit word) starting from `offset`.
    /// If you read past the current memory size, the EVM standard says it should return 0s.
    pub fn read_word(&self, offset: usize) -> U256 {
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            if offset + i < self.data.len() {
                bytes[i] = self.data[offset + i];
            }
        }
        U256::from_big_endian(&bytes)
    }

    /// Store a 32-byte (256-bit) word into memory at `offset`.
    /// This will automatically expand the volatile memory buffer with zeros if needed.
    pub fn store_word(&mut self, offset: usize, value: U256) {
        self.expand_if_needed(offset, 32);
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        
        for i in 0..32 {
            self.data[offset + i] = bytes[i];
        }
    }

    /// Expands the linear memory with zero-padding if the requested write range 
    /// goes beyond the current total capacity.
    fn expand_if_needed(&mut self, offset: usize, size: usize) {
        let required_size = offset + size;
        if required_size > self.data.len() {
            self.data.resize(required_size, 0); 
        }
    }

    /// Returns the current size of the memory array in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_expansion_and_store() {
        let mut mem = Memory::new();
        assert_eq!(mem.len(), 0);
        
        // Write 0x01 to offset 0. Should expand to exactly 32 bytes.
        mem.store_word(0, U256::from(1));
        assert_eq!(mem.len(), 32);
        assert_eq!(mem.read_word(0), U256::from(1));

        // Write to an offset way out in the future. 
        // EVM memory is contiguous, so the gap gets padded with zeros.
        mem.store_word(100, U256::from(5));
        assert_eq!(mem.len(), 132);
        assert_eq!(mem.read_word(100), U256::from(5));
        
        // Ensure what we jumped over is indeed zeros
        assert_eq!(mem.read_word(50), U256::from(0));
    }
}
