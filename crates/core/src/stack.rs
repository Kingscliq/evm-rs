use evm_shared::EvmError;
use primitive_types::U256;

const MAX_STACK_SIZE: usize = 1024;

#[derive(Debug, Clone, Default)]
pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(MAX_STACK_SIZE),
        }
    }

    /// Pushes a 256-bit value onto the stack
    pub fn push(&mut self, value: U256) -> Result<(), EvmError> {
        if self.data.len() >= MAX_STACK_SIZE {
            return Err(EvmError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    /// Pops the top value off the stack
    pub fn pop(&mut self) -> Result<U256, EvmError> {
        self.data.pop().ok_or(EvmError::StackUnderflow)
    }

    /// Peeks at an element `depth` from the top (0 is the top element)
    pub fn peek(&self, depth: usize) -> Result<U256, EvmError> {
        if depth >= self.data.len() {
            return Err(EvmError::StackUnderflow);
        }
        Ok(self.data[self.data.len() - 1 - depth])
    }

    /// Returns the current number of items on the stack
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();
        stack.push(U256::from(10)).unwrap();
        stack.push(U256::from(20)).unwrap();

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop().unwrap(), U256::from(20));
        assert_eq!(stack.pop().unwrap(), U256::from(10));
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), Err(EvmError::StackUnderflow));
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new();
        for _ in 0..1024 {
            stack.push(U256::from(1)).unwrap();
        }
        assert_eq!(stack.push(U256::from(2)), Err(EvmError::StackOverflow));
    }
}
