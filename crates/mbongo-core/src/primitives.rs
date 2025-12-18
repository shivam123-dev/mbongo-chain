use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 32-byte hash used across headers and roots.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// Returns the zero hash (all bytes zero).
    pub const fn zero() -> Self { Self([0u8; 32]) }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl std::str::FromStr for Hash {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s).map_err(|e| e.to_string())?;
        if bytes.len() != 32 {
            return Err(format!("expected 32 bytes, got {}", bytes.len()));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Hash(arr))
    }
}

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// Opaque transaction bytes.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Transaction(pub Vec<u8>);

impl Serialize for Transaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("0x{}", hex::encode(&self.0)))
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        Ok(Transaction(bytes))
    }
}

/// Block header containing chain linkage and commitments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Hash of the parent block.
    pub parent_hash: Hash,
    /// State root after executing this block.
    pub state_root: Hash,
    /// Merkle-like commitment to the body transactions.
    pub transactions_root: Hash,
    /// Unix timestamp (seconds).
    pub timestamp: u64,
    /// Block height (genesis = 0).
    pub height: u64,
}

/// Block body containing ordered transactions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BlockBody {
    /// Ordered list of transactions included in the block.
    pub transactions: Vec<Transaction>,
}

/// Full block with header and body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    /// Header with metadata and commitments.
    pub header: BlockHeader,
    /// Body with transactions.
    pub body: BlockBody,
}

/// Compute a deterministic commitment over transactions.
/// This is a simple Blake3 hash over length-prefixed transaction bytes.
pub fn compute_transactions_root(txs: &[Transaction]) -> Hash {
    use blake3::Hasher;
    let mut hasher = Hasher::new();
    for tx in txs {
        let len = tx.0.len() as u32;
        hasher.update(&len.to_le_bytes());
        hasher.update(&tx.0);
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(hasher.finalize().as_bytes());
    Hash(out)
}