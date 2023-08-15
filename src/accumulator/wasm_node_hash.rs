use bitcoin_hashes::hex;
use bitcoin_hashes::sha256;
use bitcoin_hashes::sha512_256;
use bitcoin_hashes::Hash;
use bitcoin_hashes::HashEngine;
use std::str::FromStr;
use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
    ops::Deref,
};
use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[wasm_bindgen]
#[repr(C)]
pub struct NodeHash {
    ty: NodeHashTy,
    val: Option<[u8; 32]>,
}
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum NodeHashTy {
    Empty,
    Placeholder,
    Some,
}
impl Deref for NodeHash {
    fn deref(&self) -> &Self::Target {
        if let Some(ref val) = self.val {
            val
        } else {
            &[0; 32]
        }
    }
    type Target = [u8; 32];
}

impl Default for NodeHash {
    fn default() -> Self {
        NodeHash {
            ty: NodeHashTy::Empty,
            val: None,
        }
    }
}

impl Display for NodeHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let NodeHashTy::Some = self.ty {
            let inner = self.deref();
            let mut s = String::new();
            for byte in inner.iter() {
                s.push_str(&format!("{:02x}", byte));
            }
            write!(f, "{}", s)
        } else {
            write!(f, "empty")
        }
    }
}
impl Debug for NodeHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let NodeHashTy::Some = self.ty {
            let inner = self.deref();
            let mut s = String::new();
            for byte in inner.iter() {
                s.push_str(&format!("{:02x}", byte));
            }
            write!(f, "{}", s)
        } else {
            write!(f, "empty")
        }
    }
}
impl From<sha512_256::Hash> for NodeHash {
    fn from(hash: sha512_256::Hash) -> Self {
        NodeHash {
            ty: NodeHashTy::Some,
            val: Some(hash.to_byte_array()),
        }
    }
}
impl From<[u8; 32]> for NodeHash {
    fn from(hash: [u8; 32]) -> Self {
        NodeHash {
            ty: NodeHashTy::Some,
            val: Some(hash),
        }
    }
}
impl From<&[u8; 32]> for NodeHash {
    fn from(hash: &[u8; 32]) -> Self {
        NodeHash {
            ty: NodeHashTy::Some,
            val: Some(*hash),
        }
    }
}
#[cfg(test)]
impl TryFrom<&str> for NodeHash {
    type Error = hex::Error;
    fn try_from(hash: &str) -> Result<Self, Self::Error> {
        // This implementation is useful for testing, as it allows to create empty hashes
        // from the string of 64 zeros. Without this, it would be impossible to express this
        // hash in the test vectors.
        if hash == "0000000000000000000000000000000000000000000000000000000000000000" {
            return Ok(NodeHash::default());
        }
        let hash = hex::FromHex::from_hex(hash)?;
        Ok(NodeHash {
            ty: NodeHashTy::Some,
            val: Some(hash),
        })
    }
}

#[cfg(not(test))]
impl TryFrom<&str> for NodeHash {
    type Error = hex::Error;
    fn try_from(hash: &str) -> Result<Self, Self::Error> {
        let inner: [u8; 32] = hex::FromHex::from_hex(hash)?;
        Ok(NodeHash::from(inner))
    }
}
impl From<&[u8]> for NodeHash {
    fn from(hash: &[u8]) -> Self {
        let mut inner: [u8; 32] = [0; 32];
        inner.copy_from_slice(hash);
        NodeHash::from(inner)
    }
}

impl From<sha256::Hash> for NodeHash {
    fn from(hash: sha256::Hash) -> Self {
        NodeHash::from(hash.to_byte_array())
    }
}
impl FromStr for NodeHash {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NodeHash::try_from(s)
    }
    type Err = hex::Error;
}
impl NodeHash {
    /// Tells whether this hash is empty. We use empty hashes throughout the code to represent
    /// leaves we want to delete.
    pub fn is_empty(&self) -> bool {
        match self.ty {
            NodeHashTy::Empty => true,
            _ => false,
        }
    }
    /// Creates a new NodeHash from a 32 byte array.
    /// # Example
    /// ```
    /// use rustreexo::accumulator::node_hash::NodeHash;
    /// let hash = NodeHash::new([0; 32]);
    /// assert_eq!(hash.to_string().as_str(), "0000000000000000000000000000000000000000000000000000000000000000");
    /// ```
    pub fn new(inner: [u8; 32]) -> Self {
        NodeHash::from(inner)
    }
    /// Creates an empty hash. This is used to represent leaves we want to delete.
    /// # Example
    /// ```
    /// use rustreexo::accumulator::node_hash::NodeHash;
    /// let hash = NodeHash::empty();
    /// assert!(hash.is_empty());
    /// ```
    pub fn empty() -> Self {
        NodeHash::default()
    }
    /// parent_hash return the merkle parent of the two passed in nodes.
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use rustreexo::accumulator::node_hash::NodeHash;
    /// let left = NodeHash::new([0; 32]);
    /// let right = NodeHash::new([1; 32]);
    /// let parent = NodeHash::parent_hash(&left, &right);
    /// let expected_parent = NodeHash::from_str("34e33ca0c40b7bd33d28932ca9e35170def7309a3bf91ecda5e1ceb067548a12").unwrap();
    /// assert_eq!(parent, expected_parent);
    /// ```
    pub fn parent_hash(left: &NodeHash, right: &NodeHash) -> NodeHash {
        let mut hash = sha512_256::Hash::engine();
        hash.input(&**left);
        hash.input(&**right);
        sha512_256::Hash::from_engine(hash).into()
    }

    /// Returns a arbitrary placeholder hash that is unlikely to collide with any other hash.
    /// We use this while computing roots to destroy. Don't confuse this with an empty hash.
    pub const fn placeholder() -> Self {
        NodeHash {
            ty: NodeHashTy::Placeholder,
            val: None,
        }
    }
}
