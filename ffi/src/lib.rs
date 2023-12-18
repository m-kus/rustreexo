use std::{error::Error, fmt::Display};

pub use rustreexo::accumulator::{
    node_hash::NodeHash as RustreexoNodeHash, proof::Proof as RustreexoProof,
    stump::Stump as RustreexoStump,
};

struct NodeHash {
    bytes: Vec<u8>,
}

struct Stump {
    inner: RustreexoStump,
}

#[derive(Debug, Clone)]
enum RustreexoError {
    GenericError(String),
}

impl Display for RustreexoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustreexoError::GenericError(e) => write!(f, "{e}"),
        }
    }
}

impl From<String> for RustreexoError {
    fn from(value: String) -> Self {
        RustreexoError::GenericError(value)
    }
}

impl Error for RustreexoError {}

struct Proof {
   proof: RustreexoProof
}

impl Proof {
    fn new() -> Proof {
       Proof { proof: RustreexoProof::default() }
    }
}

impl Stump {
    pub fn new() -> Stump {
        Stump {
            inner: RustreexoStump::default(),
        }
    }

    pub fn modify(
        &self,
        utxos: Vec<NodeHash>,
        stxos: Vec<NodeHash>,
        proof: &Proof,
    ) -> Result<(), String> {
        let utxos = utxos
            .iter()
            .map(|hash| RustreexoNodeHash::from(hash.bytes.as_slice()))
            .collect::<Vec<_>>();

        let stxos = stxos
            .iter()
            .map(|hash| RustreexoNodeHash::from(hash.bytes.as_slice()))
            .collect::<Vec<_>>();

        self.inner.modify(&utxos, &stxos, &proof.proof)?;

        Ok(())
    }
}

uniffi::include_scaffolding!("rustreexo_bindings");
