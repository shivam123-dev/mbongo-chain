//! Core blockchain primitives for Mbongo Chain.
//!
//! This crate provides the foundational types and utilities used throughout
//! the Mbongo Chain blockchain, including:
//! - Block and transaction primitives
//! - Cryptographic helpers (hashing)
//!
//! # Block Primitives
//!
//! The `Block` type models a blockchain block consisting of a header and body.
//! The header contains chain-linkage, commitment roots and metadata; the body
//! contains the ordered list of transactions.
//!
//! ```rust
//! use mbongo_core::{Block, BlockHeader, BlockBody, Hash, Transaction};
//!
//! // Build a simple block with two transactions (opaque bytes)
//! let txs = vec![Transaction(vec![1,2,3]), Transaction(vec![4,5])];
//! let header = BlockHeader {
//!     parent_hash: Hash::zero(),
//!     state_root: Hash::zero(),
//!     transactions_root: mbongo_core::compute_transactions_root(&txs),
//!     timestamp: 1_700_000_000,
//!     height: 1,
//! };
//! let body = BlockBody { transactions: txs };
//! let _block = Block { header, body };
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod primitives;

pub use primitives::{compute_transactions_root, Block, BlockBody, BlockHeader, Hash, Transaction};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;

    #[test]
    fn hash_zero_and_hex_roundtrip() {
        let h = Hash::zero();
        let s = h.to_string();
        assert_eq!(s, "0x".to_string() + &"0".repeat(64));
        let parsed: Hash = s.parse().unwrap();
        assert_eq!(parsed, h);
        let ser = json::to_string(&h).unwrap();
        let de: Hash = json::from_str(&ser).unwrap();
        assert_eq!(de, h);
    }

    #[test]
    fn block_serde_roundtrip() {
        let txs = vec![Transaction(vec![1, 2, 3]), Transaction(vec![])];
        let header = BlockHeader {
            parent_hash: Hash::zero(),
            state_root: Hash::zero(),
            transactions_root: compute_transactions_root(&txs),
            timestamp: 123,
            height: 7,
        };
        let block = Block { header, body: BlockBody { transactions: txs } };
        let s = json::to_string(&block).unwrap();
        let round: Block = json::from_str(&s).unwrap();
        assert_eq!(round.header.height, 7);
        assert_eq!(round.body.transactions.len(), 2);
    }

    #[test]
    fn transactions_root_changes_with_body() {
        let a = vec![Transaction(vec![1])];
        let b = vec![Transaction(vec![2])];
        let ra = compute_transactions_root(&a);
        let rb = compute_transactions_root(&b);
        assert_ne!(ra, rb);
    }
}
