use primitive_types::U256;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub address: U256,
    pub caller: U256,
    pub origin: U256,
    pub value: U256,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            address: U256::from(0x1000),
            caller: U256::from(0x2000),
            origin: U256::from(0x2000),
            value: U256::zero(),
        }
    }
}
