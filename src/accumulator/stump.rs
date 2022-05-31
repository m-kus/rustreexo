use super::types;

#[derive(Debug, Clone)]
pub struct Stump {
  leafs: u64,
  roots: Vec<bitcoin_hashes::sha256::Hash>
}

impl Stump {
  /// Creates an empty Stump
  pub fn new() -> Self {
    Stump {
      leafs: 0,
      roots: Vec::new()
    }
  }
  /// Modify is the external API to change the accumulator state. Since order
  /// matters, you can only modify, providing a list of utxos to be added, 
  /// and txos (@TODO) to be removed, along with it's proof. Either may be
  /// empty.
  ///# Example
  /// ```
  ///   use rustreexo::accumulator::stump::Stump;
  ///   let mut s = Stump::new();
  ///   let utxos = vec![];
  ///   let stxos = vec![];
  ///   s.modify(&utxos, &stxos);
  /// ```
  pub fn modify(&mut self, utxos: &Vec<bitcoin_hashes::sha256::Hash>, _stxos: &Vec<bitcoin_hashes::sha256::Hash>) {
    //remove
    self.add(utxos);
  }

  /// Rewinds old tree state, this should be used in case of reorgs.
  /// Takes the ownership over `old_state`.
  ///# Example
  /// ```
  ///   use rustreexo::accumulator::stump::Stump;
  ///   let mut s_old = Stump::new();
  ///   let mut s_new = Stump::new();
  ///   
  ///   s_old.modify(&vec![], &vec![]);
  ///   s_new = s_old.clone();
  ///   s_new.modify(&vec![], &vec![]);
  ///   
  ///   // A reorg happened
  ///   
  ///   s_new.undo(s_old);  
  ///```
  pub fn undo(&mut self, old_state: Stump) {
    self.leafs = old_state.leafs;
    self.roots = old_state.roots;
  }

  /// Adds new leafs into the root
  fn add(&mut self, utxos: &Vec<bitcoin_hashes::sha256::Hash>) {
    for i in utxos.iter() {
      self.add_single(*i);
    }
  }

  fn add_single(&mut self, node: bitcoin_hashes::sha256::Hash) {
    let mut h = 0;
    // Iterates over roots, if we find a root that is not empty, we concatenate with
    // the one we are adding and create new root, leaving this position empty. Stops
    // when find an empty root.

    // You can say if a root is empty, by looking a the binary representations of the
    // number of leafs. If the h'th bit is one, then this position is occupied, empty 
    // otherwise.
    let mut to_add = node;
    while (self.leafs >> h) & 1 == 1 {
      let root = self.roots.pop();
      if let Some(root) = root {
        to_add = types::parent_hash(&root, &to_add);
      }
      h += 1;
    }

    self.roots.push(to_add);

    self.leafs += 1;
  }
}



#[cfg(test)]
mod test {
  use std::vec;
  use bitcoin_hashes::{sha256, Hash, HashEngine};
  use super::Stump;

  
  #[test]
  // Make a few simple tests about stump creation
  fn test_stump() {
    let s = Stump::new();
    assert!(s.leafs == 0);
    assert!(s.roots.len() == 0);
  }

  fn hash_from_u8(value: u8) -> sha256::Hash {
    let mut engine = bitcoin_hashes::sha256::Hash::engine();

    engine.input(&[value]);

    sha256::Hash::from_engine(engine)
  }
  #[test]
  fn test_add() {
    let mut s = Stump::new();
    let test_values:Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    /* Hardcoded test cases generated here: https://go.dev/play/p/pODpvB9NXAZ */
    let fingerprints = [
                                    /* Leafs */
                                    [0x6e, 0x34], [0x4b, 0xf5], [0xdb, 0xc1], [0x08, 0x4f],
                                    [0xe5, 0x2d], [0xe7, 0x7b], [0x67, 0x58], [0xca, 0x35],
                                    
                                    /* ... Stump don't stores internal nodes ...*/

                                    /* Root */
                                    [0xb1, 0x51]
                                  ];

    let mut hashes = vec![];

    for i in test_values {
      hashes.push(hash_from_u8(i));
      assert_eq!(fingerprints[i as usize], hashes[i as usize][0..2]);
    }

    s.modify(&hashes, &vec![]);
    
    assert_eq!(fingerprints[hashes.len()], s.roots[0][0..2]);
  }

}