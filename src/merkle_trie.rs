use sha2::{Digest, Sha256};

/// A simple Merkle Tree implementation using SHA-256 hashing
pub struct MerkleTree {
    /// The root hash of the Merkle tree
    root: Vec<u8>,
    /// All tree node hashes in level order
    nodes: Vec<Vec<u8>>,
    /// Number of leaf nodes
    leaf_count: usize,
}

impl MerkleTree {
    /// Create a new Merkle tree from a list of data items
    pub fn new<T: AsRef<[u8]>>(data: &[T]) -> Self {
        if data.is_empty() {
            panic!("Cannot create Merkle tree from empty data");
        }

        let mut nodes = Vec::new();
        
        // Create leaf nodes (level 0)
        let mut current_level = Vec::new();
        for item in data {
            let hash = Self::hash(item.as_ref());
            current_level.push(hash);
        }
        
        let leaf_count = current_level.len();
        nodes.push(current_level);
        
        // Build tree upwards until we reach the root
        while nodes.last().unwrap().len() > 1 {
            let last_level = nodes.last().unwrap();
            let mut new_level = Vec::new();
            
            // Combine pairs of nodes
            for i in (0..last_level.len()).step_by(2) {
                if i + 1 < last_level.len() {
                    // Combine two child nodes
                    let mut combined = last_level[i].clone();
                    combined.extend_from_slice(&last_level[i + 1]);
                    let parent_hash = Self::hash(&combined);
                    new_level.push(parent_hash);
                } else {
                    // Odd number of nodes, promote the last one
                    new_level.push(last_level[i].clone());
                }
            }
            
            nodes.push(new_level);
        }
        
        // The root is the last node in the last level
        let root = nodes.last().unwrap()[0].clone();
        
        MerkleTree {
            root,
            nodes: nodes.into_iter().flatten().collect(),
            leaf_count,
        }
    }
    
    /// Get the root hash of the Merkle tree
    pub fn root_hash(&self) -> &[u8] {
        &self.root
    }
    
    /// Generate a Merkle proof for a leaf at the given index
    pub fn generate_proof(&self, leaf_index: usize) -> MerkleProof {
        if leaf_index >= self.leaf_count {
            panic!("Leaf index out of bounds");
        }
        
        let mut proof = Vec::new();
        let mut index = leaf_index;
        
        // For each level (except the root), add the sibling node to the proof
        for level in 0..self.nodes.len() - 1 {
            let level_nodes = &self.nodes[level];
            let is_right = index % 2 == 1;
            let sibling_idx = if is_right { index - 1 } else { index + 1 };
            
            if sibling_idx < level_nodes.len() {
                proof.push((level_nodes[sibling_idx].clone(), is_right));
            }
            
            // Move to parent index for next level
            index /= 2;
        }
        
        // MerkleProof {
        //     proof,
        //     leaf_hash: self.nodes[0][leaf_index].clone(),
        //     root_hash: self.root.clone(),
        // }
        todo!()
    }
    
    /// Helper function to compute SHA-256 hash
    fn hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

/// A proof that a leaf is included in the Merkle tree
pub struct MerkleProof {
    /// The proof nodes, each with a flag indicating if it's a right sibling
    proof: Vec<(Vec<u8>, bool)>,
    /// The hash of the leaf being proven
    leaf_hash: Vec<u8>,
    /// The root hash of the tree
    root_hash: Vec<u8>,
}

impl MerkleProof {
    /// Verify the Merkle proof
    pub fn verify<T: AsRef<[u8]>>(&self, data: T) -> bool {
        let leaf_hash = MerkleTree::hash(data.as_ref());
        
        // Check if the leaf hash matches
        if leaf_hash != self.leaf_hash {
            return false;
        }
        
        let mut current_hash = leaf_hash;
        
        // Traverse up the tree using the proof
        for (sibling, is_right) in &self.proof {
            let mut combined = Vec::new();
            
            if *is_right {
                // Current hash is left, sibling is right
                combined.extend_from_slice(&current_hash);
                combined.extend_from_slice(sibling);
            } else {
                // Current hash is right, sibling is left
                combined.extend_from_slice(sibling);
                combined.extend_from_slice(&current_hash);
            }
            
            current_hash = MerkleTree::hash(&combined);
        }
        
        // Check if we've arrived at the root
        current_hash == self.root_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merkle_tree() {
        let data = vec!["a", "b", "c", "d"];
        let tree = MerkleTree::new(&data);
        
        // Verify that the tree has the correct structure
        assert_eq!(tree.nodes.len(), 3); // 3 levels: leaves, internal, root
        assert_eq!(tree.nodes[0].len(), 4); // 4 leaves
        assert_eq!(tree.nodes[1].len(), 2); // 2 internal nodes
        assert_eq!(tree.nodes[2].len(), 1); // 1 root
    }
    
    #[test]
    fn test_merkle_proof() {
        let data = vec!["a", "b", "c", "d"];
        let tree = MerkleTree::new(&data);
        
        // Generate and verify a proof for each item
        for (i, item) in data.iter().enumerate() {
            let proof = tree.generate_proof(i);
            assert!(proof.verify(item));
            
            // Verify that the proof fails for different data
            if i > 0 {
                assert!(!proof.verify(data[i-1]));
            }
        }
    }
}
