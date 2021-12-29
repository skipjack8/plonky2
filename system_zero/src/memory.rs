pub struct TransactionMemory {
    pub calls: Vec<ContractMemory>,
}

impl Default for TransactionMemory {
    fn default() -> Self {
        Self { calls: vec![] }
    }
}

/// A virtual memory space specific to the current contract call.
pub struct ContractMemory {
    pub code: MemorySegment,
    pub main: MemorySegment,
    pub calldata: MemorySegment,
    pub returndata: MemorySegment,
}

pub struct MemorySegment {
    pub content: Vec<u8>,
}
