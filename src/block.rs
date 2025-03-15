use crate::merkle_trie::MerkleTree;

pub struct Block {
    header: BlockHeader,
    transactions: Vec<Vec<u8>>,
    merkle_tree: MerkleTree,
}

pub struct BlockHeader {
    version: u32,
    prev_block_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    // timestamp: u64,
    nonce: u64,
}

impl Block {
    // Create a new block with given transactions and previous block hash
    pub fn new(transactions: Vec<Vec<u8>>, prev_block_hash: Vec<u8>) -> Self {
        // Create Merkle tree from transactions
        let merkle_tree = MerkleTree::new(&transactions);
        
        // Create block header
        let header = BlockHeader {
            version: 1,
            prev_block_hash,
            merkle_root: merkle_tree.root_hash().to_vec(),
            // timestamp: current_timestamp(),
            nonce: 0,
        };
        
        Block {
            header,
            transactions,
            merkle_tree,
        }
    }
    
    // Calculate the hash of this block
    pub fn hash(&self) -> Vec<u8> {
        // Serialize header and hash it
        let serialized = self.serialize_header();
        MerkleTree::hash(&serialized)
    }
    
    // Helper function to serialize the header for hashing
    fn serialize_header(&self) -> Vec<u8> {
        // Simple serialization by concatenating fields
        let mut buffer = Vec::new();
        
        // Add version
        buffer.extend_from_slice(&self.header.version.to_le_bytes());
        // Add prev block hash
        buffer.extend_from_slice(&self.header.prev_block_hash);
        // Add merkle root
        buffer.extend_from_slice(&self.header.merkle_root);
        // Add timestamp
        // buffer.extend_from_slice(&self.header.timestamp.to_le_bytes());
        // Add nonce
        buffer.extend_from_slice(&self.header.nonce.to_le_bytes());
        
        buffer
    }
    
    // Helper function to get current timestamp (seconds since epoch)
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
    
    // Mine the block until its hash has the required number of leading zeros
    pub fn mine(&mut self, difficulty: usize) {
        // let target = vec![0; difficulty / 8];
        let remainder = difficulty % 8;
        let mask = if remainder > 0 { 0xff >> remainder } else { 0 };
        
        loop {
            let hash = self.hash();
            
            // Check if hash meets difficulty (has enough leading zeros)
            let meets_difficulty = hash.iter().take(difficulty / 8).all(|&b| b == 0) &&
                (remainder == 0 || (hash[difficulty / 8] & !mask) == 0);
                
            if meets_difficulty {
                break;
            }
            
            // Increment nonce and try again
            self.header.nonce += 1;
        }
    }
    
    // Accessors
    pub fn merkle_root(&self) -> &[u8] {
        &self.header.merkle_root
    }
    
    pub fn prev_block_hash(&self) -> &[u8] {
        &self.header.prev_block_hash
    }
    
    // pub fn timestamp(&self) -> u64 {
    //     self.header.timestamp
    // }
    
    pub fn nonce(&self) -> u64 {
        self.header.nonce
    }
    
    pub fn transactions(&self) -> &[Vec<u8>] {
        &self.transactions
    }
}
