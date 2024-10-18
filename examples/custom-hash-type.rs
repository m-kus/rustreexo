//! All data structures in this library are generic over the hash type used, defaulting to
//! [BitcoinNodeHash](crate::accumulator::node_hash::BitcoinNodeHash), the one used by Bitcoin
//! as defined by the utreexo spec. However, if you need to use a different hash type, you can
//! implement the [NodeHash](crate::accumulator::node_hash::NodeHash) trait for it, and use it
//! with the accumulator data structures.
//!
//! This example shows how to use a custom hash type based on the Poseidon hash function. The
//! [Poseidon Hash](https://eprint.iacr.org/2019/458.pdf) is a hash function that is optmized
//! for zero-knowledge proofs, and is used in projects like ZCash and StarkNet.
//! If you want to work with utreexo proofs in zero-knowledge you may want to use this instead
//! of our usual sha512-256 that we use by default, since that will give you smaller circuits.
//! This example shows how to use both the [Pollard](crate::accumulator::pollard::Pollard) and
//! proofs with a custom hash type. The code here should be pretty much all you need to do to
//! use your custom hashes, just tweak the implementation of
//! [NodeHash](crate::accumulator::node_hash::NodeHash) for your hash type.

use std::str::FromStr;

use rustreexo::accumulator::node_hash::AccumulatorHash;
use rustreexo::accumulator::pollard::Pollard;
use serde::Deserialize;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use starknet_crypto::poseidon_hash_many;
use starknet_crypto::Felt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// We need a stateful wrapper around the actual hash, this is because we use those different
/// values inside our accumulator. Here we use an enum to represent the different states, you
/// may want to use a struct with more data, depending on your needs.
enum PoseidonHash {
    /// This means this holds an actual value
    ///
    /// It usually represents a node in the accumulator that haven't been deleted.
    Hash(Felt),
    /// Placeholder is a value that haven't been deleted, but we don't have the actual value.
    /// The only thing that matters about it is that it's not empty. You can implement this
    /// the way you want, just make sure that [NodeHash::is_placeholder] and [NodeHash::placeholder]
    /// returns sane values (that is, if we call [NodeHash::placeholder] calling [NodeHash::is_placeholder]
    /// on the result should return true).
    Placeholder,
    /// This is an empty value, it represents a node that was deleted from the accumulator.
    ///
    /// Same as the placeholder, you can implement this the way you want, just make sure that
    /// [NodeHash::is_empty] and [NodeHash::empty] returns sane values.
    Empty,
}

// you'll need to implement Display for your hash type, so you can print it.
impl std::fmt::Display for PoseidonHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoseidonHash::Hash(h) => write!(f, "Hash({})", h),
            PoseidonHash::Placeholder => write!(f, "Placeholder"),
            PoseidonHash::Empty => write!(f, "Empty"),
        }
    }
}

// this is the implementation of the NodeHash trait for our custom hash type. And it's the only
// thing you need to do to use your custom hash type with the accumulator data structures.
impl AccumulatorHash for PoseidonHash {
    // returns a new placeholder type such that is_placeholder returns true
    fn placeholder() -> Self {
        PoseidonHash::Placeholder
    }

    // returns an empty hash such that is_empty returns true
    fn empty() -> Self {
        PoseidonHash::Empty
    }

    // returns true if this is a placeholder. This should be true iff this type was created by
    // calling placeholder.
    fn is_placeholder(&self) -> bool {
        matches!(self, PoseidonHash::Placeholder)
    }

    // returns true if this is an empty hash. This should be true iff this type was created by
    // calling empty.
    fn is_empty(&self) -> bool {
        matches!(self, PoseidonHash::Empty)
    }

    // used for serialization, writes the hash to the writer
    //
    // if you don't want to use serialization, you can just return an error here.
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            PoseidonHash::Hash(h) => writer.write_all(&h.to_bytes_be()),
            PoseidonHash::Placeholder => writer.write_all(&[0u8; 32]),
            PoseidonHash::Empty => writer.write_all(&[0u8; 32]),
        }
    }

    // used for deserialization, reads the hash from the reader
    //
    // if you don't want to use serialization, you can just return an error here.
    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let mut buf = [0u8; 32];
        reader.read_exact(&mut buf)?;
        if buf.iter().all(|&b| b == 0) {
            Ok(PoseidonHash::Empty)
        } else {
            Ok(PoseidonHash::Hash(Felt::from_bytes_be(&buf)))
        }
    }

    // the main thing about the hash type, it returns the next node's hash, given it's children.
    // The implementation of this method is highly consensus critical, so everywhere should use the
    // exact same algorithm to calculate the next hash. Rustreexo won't call this method, unless
    // **both** children are not empty.
    fn parent_hash(left: &Self, right: &Self) -> Self {
        if let (PoseidonHash::Hash(left), PoseidonHash::Hash(right)) = (left, right) {
            return PoseidonHash::Hash(poseidon_hash_many(&[*left, *right]));
        }

        // This should never happen, since rustreexo won't call this method unless both children
        // are not empty.
        unreachable!()
    }
}

// fn main() {
//     // Create a vector with two utxos that will be added to the Pollard
//     let elements = vec![
//         PoseidonHash::Hash(Felt::from(1)),
//         PoseidonHash::Hash(Felt::from(2)),
//     ];

//     // Create a new Pollard, and add the utxos to it
//     let mut p = Pollard::<PoseidonHash>::new_with_hash();
//     p.modify(&elements, &[]).unwrap();

//     // Create a proof that the first utxo is in the Pollard
//     let proof = p.prove(&[elements[0]]).unwrap();

//     // check that the proof has exactly one target
//     assert_eq!(proof.n_targets(), 1);
//     // check that the proof is what we expect
//     assert!(p.verify(&proof, &[elements[0]]).unwrap());
// }

#[derive(Debug, Deserialize)]
struct TestData {
    init_add: Vec<u8>,
    init_del: Vec<u8>,
    roots: Vec<String>,
    leaves: u64,
    /// New data to add
    additional_preimages: Vec<u64>,
    /// The hash of all targets to be deleted
    del_hashes: Vec<String>,
    /// The hashes that are used to recompute a given Merkle path to the root
    proof_hashes: Vec<String>,
    /// Which nodes are being proven, in this case, they'll be deleted
    proof_targets: Vec<u64>,
    /// Here are the expected values:
    /// During addition, we create those nodes
    new_add_pos: Vec<u64>,
    new_add_hash: Vec<String>,
    /// And during deletion, we destroy or update those
    new_del_pos: Vec<u64>,
    new_del_hashes: Vec<String>,
    to_destroy: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct CachedTestData {
    /// Blocks contains new utxos and utxos that are being deleted
    update: UpdatedData,
    /// The proof we have for our wallet's utxos
    cached_proof: JsonProof,
    init_add: Vec<u8>,
    init_del: Vec<u8>,
    /// A initial set of roots, may be empty for starting with an empty stump
    initial_roots: Vec<String>,
    /// The number of leaves in the initial Stump
    initial_leaves: u64,
    /// The hash of all wallet's utxo
    cached_hashes: Vec<String>,
    /// The indexes of all the new utxos to cache
    remembers: Vec<u64>,
    /// After we update our stump, which roots we expect?
    expected_roots: Vec<String>,
    /// After we update the proof, the proof's target should be this
    expected_targets: Vec<u64>,
    /// After we update the proof, the cached hashes should be this
    expected_cached_hashes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JsonProof {
    targets: Vec<u64>,
    hashes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UpdatedData {
    /// The newly created utxo to be added to our accumulator
    adds: Vec<u64>,
    /// The proof for all destroyed utxos
    proof: JsonProof,
    /// The hash of all destroyed utxos
    del_hashes: Vec<String>,
}

#[derive(Deserialize)]
struct TestsJSON {
    insertion_tests: Vec<TestCase>,
    deletion_tests: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
struct TestCase {
    leaf_preimages: Vec<u8>,
    target_values: Option<Vec<u64>>,
    expected_roots: Vec<String>,
    proofhashes: Option<Vec<String>>,
}

#[derive(Serialize, Debug)]
struct Output {
    state: State,
    proof: BatchProof,
    leaves_to_del: Vec<PoseidonHash>,
    leaves_to_add: Vec<PoseidonHash>,
    expected_state: State,
}

#[derive(Default, Serialize, Debug)]
struct State {
    roots: Vec<Root>,
    leaves: u64,
}

#[derive(Default, Serialize, Debug)]
struct BatchProof {
    nodes: Vec<PoseidonHash>,
    targets: Vec<u64>,
}

fn main() {
    // test_cases
    let contents = std::fs::read_to_string("test_values/test_cases.json")
        .expect("Something went wrong reading the file");

    let tests = serde_json::from_str::<TestsJSON>(contents.as_str())
        .expect("JSON deserialization error");

    for (i, test_case) in tests.deletion_tests.into_iter().enumerate() {
        handle_deletion_test_case(test_case, i);
    }

    for (i, test_case) in tests.insertion_tests.into_iter().enumerate() {
        handle_insertion_test_case(test_case, i);
    }

    // update_data_tests
    let contents = std::fs::read_to_string("test_values/update_data_tests.json")
        .expect("Something went wrong reading the file");

    let tests = serde_json::from_str::<Vec<TestData>>(contents.as_str())
        .expect("JSON deserialization error");

    for (i, test_case) in tests.into_iter().enumerate() {
        handle_update_data_test_case(test_case, i)
    }

    // cached_proof_tests
    let contents = std::fs::read_to_string("test_values/cached_proof_tests.json")
        .expect("Something went wrong reading the file");

    let tests = serde_json::from_str::<Vec<CachedTestData>>(contents.as_str())
        .expect("JSON deserialization error");

    for (i, test_case) in tests.into_iter().enumerate() {
        handle_cached_proof_test_case(test_case, i)
    }
}

fn handle_deletion_test_case(test_case: TestCase, text_idx: usize) {
    let mut p = Pollard::<PoseidonHash>::new_with_hash();
    let add: Vec<PoseidonHash> = test_case.leaf_preimages.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    p.modify(&add, &[]).unwrap();
    
    let state = pollard_state(&p);

    let del = test_case
        .target_values
        .unwrap()
        .iter()
        .map(|&x| PoseidonHash::Hash(x.into()))
        .collect::<Vec<_>>();
    let proof = p.prove(&del).unwrap();

    p.modify(&[], &del).unwrap();

    let output = Output {
        state,
        proof: BatchProof {
            nodes: proof.hashes,
            targets: proof.targets,
        },
        leaves_to_del: del,
        leaves_to_add: vec![],
        expected_state: pollard_state(&p),
    };
    
    let content = serde_json::to_string_pretty(&output).unwrap();
    std::fs::write(format!("test_data/deletion_test_case_{text_idx}.json"), content).unwrap();
}

fn handle_insertion_test_case(test_case: TestCase, text_idx: usize) {
    let mut p = Pollard::<PoseidonHash>::new_with_hash();
    let state = pollard_state(&p);

    let add: Vec<PoseidonHash> = test_case.leaf_preimages.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    p.modify(&add, &[]).unwrap();

    let output = Output {
        state,
        proof: BatchProof::default(),
        leaves_to_del: vec![],
        leaves_to_add: add,
        expected_state: pollard_state(&p),
    };
    
    let content = serde_json::to_string_pretty(&output).unwrap();
    std::fs::write(format!("test_data/insertion_test_case_{text_idx}.json"), content).unwrap();
}

fn handle_update_data_test_case(data: TestData, text_idx: usize) {
    let mut p = Pollard::<PoseidonHash>::new_with_hash();
    let init_add: Vec<PoseidonHash> = data.init_add.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    p.modify(&init_add, &[]).unwrap();
    let init_del: Vec<PoseidonHash> = data.init_del.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    p.modify(&[], &init_del).unwrap();
    let state = pollard_state(&p);

    // action
    let add: Vec<PoseidonHash> = data.additional_preimages.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    let del: Vec<PoseidonHash> = data.proof_targets.iter().map(|&x| p.grab_node(x).unwrap().0.get_data()).collect();
    let proof = p.prove(&del).unwrap();
    p.modify(&add, &del).unwrap();

    let output = Output {
        state,
        proof: BatchProof {
            nodes: proof.hashes,
            targets: proof.targets,
        },
        leaves_to_del: vec![],
        leaves_to_add: add,
        expected_state: pollard_state(&p),
    };
    
    let content = serde_json::to_string_pretty(&output).unwrap();
    std::fs::write(format!("test_data/update_data_test_case_{text_idx}.json"), content).unwrap();
}

fn handle_cached_proof_test_case(data: CachedTestData, text_idx: usize) {
    let mut p = Pollard::<PoseidonHash>::new_with_hash();
    let init_add: Vec<PoseidonHash> = data.init_add.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    p.modify(&init_add, &[]).unwrap();
    let init_del: Vec<PoseidonHash> = data.init_del.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    p.modify(&[], &init_del).unwrap();
    let state = pollard_state(&p);

    // action
    let add: Vec<PoseidonHash> = data.update.adds.iter().map(|&x| PoseidonHash::Hash(x.into())).collect();
    let del: Vec<PoseidonHash> = data.update.proof.targets.iter().map(|&x| p.grab_node(x).unwrap().0.get_data()).collect();
    let proof = p.prove(&del).unwrap();
    p.modify(&add, &del).unwrap();

    let output = Output {
        state,
        proof: BatchProof {
            nodes: proof.hashes,
            targets: proof.targets,
        },
        leaves_to_del: vec![],
        leaves_to_add: add,
        expected_state: pollard_state(&p),
    };
    
    let content = serde_json::to_string_pretty(&output).unwrap();
    std::fs::write(format!("test_data/cached_proof_test_case_{text_idx}.json"), content).unwrap();
}

fn pollard_state(p: &Pollard<PoseidonHash>) -> State {
    State {
        roots: p.get_roots().iter().map(|x| poseidon_hash_to_root(x.get_data())).collect(),
        leaves: p.leaves,
    }
}

fn poseidon_hash_to_root(hash: PoseidonHash) -> Root {
    match hash {
        PoseidonHash::Empty => Root(None),
        PoseidonHash::Placeholder => unimplemented!(),
        PoseidonHash::Hash(felt) => Root(Some(felt))
    }
}

#[derive(Debug)]
struct Root(pub Option<Felt>);

impl Serialize for Root {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            None => {
                let mut state = serializer.serialize_struct("PoseidonHash", 1)?;
                state.serialize_field("variant_id", &1)?;
                state.end()
            }
            Some(felt) => {
                let value = serde_json::Number::from_str(&felt.to_string()).unwrap();
                let mut state = serializer.serialize_struct("PoseidonHash", 2)?;
                state.serialize_field("variant_id", &0)?;
                state.serialize_field("value", &value)?;
                state.end()
            }
        }
    }
}

impl Serialize for PoseidonHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PoseidonHash::Empty => {
                Err(serde::ser::Error::custom("Unexpected Empty variant"))
            }
            PoseidonHash::Hash(felt) => {
                serde_json::Number::from_str(&felt.to_string())
                    .map_err(serde::ser::Error::custom)?
                    .serialize(serializer)
            }
            PoseidonHash::Placeholder => {
                Err(serde::ser::Error::custom("Unexpected Placeholder variant"))
            }
        }
    }
}
