// Rustreexo

use std::{mem, fmt::Debug};

use super::{
    types,
    util,
};

use bitcoin::hashes::{sha256};

/// Pollard is the sparse representation of the utreexo forest
/// It is a collection of multitude of trees with leaves that are
/// power of two.
///
/// However, the allocated tree is always a power of two. The nodes
/// that are not necessary are kept as empty nodes.
///
/// Its structure resembles of that of a binary tree, except that
/// the pointers point to aunts - nieces, not parents - children
#[derive(Clone, Debug)]
pub struct Pollard {
    /// Roots are the top-most nodes of the tree
    /// There may be multiple roots as Utreexo is organized as a
    /// collection of perfect trees.
    pub roots: Vec<PolNode>,

    /// Total number of leaves (nodes on the bottom row) in the Pollard
    pub num_leaves: u64,
}

impl Pollard {
    /// Returns a new pollard
    pub fn new() -> Pollard {
        Pollard{roots: Vec::new(), num_leaves:0, }
    }

    /// Modify changes the Utreexo tree state given the utxos and stxos
    /// stxos are denoted by their value
    pub fn modify(&mut self, utxos: Vec<types::Leaf>, stxos: Vec<u64>) -> Result<u64, String>{
        // Order matters here. Adding then removing will result in a different
        // tree vs deleting then adding. For ease of use, only modify is visible
        // for external crates. This is consensus critical.
        Pollard::remove(self, stxos)?;
        Pollard::add(self, utxos)?;
        Ok(self.num_leaves)
    }

    pub fn add(&mut self, adds: Vec<types::Leaf>) -> Result<u64, String> {
        // General algo goes:
        // 1 make a new node & assign data (no nieces; at bottom)
        // 2 if this node is on a row where there's already a root,
        // then swap nieces with that root, hash the two datas, and build a new
        // node 1 higher pointing to them.
        // goto 2.

        for add in adds {
            //if add.remember {
            //    // TODO Should cache the add data
            //}
            Pollard::add_single(self, add.hash, false);
        }
        Ok(self.num_leaves)
    }
    // recurse from the right side of the tree until we hit a tree with no root
    // Destroys roots along the way
    fn create_root(pol: &mut Pollard, mut node: PolNode, num_leaves: u64) -> PolNode {
        if num_leaves & 1 == 1 {
            // If num_leaves & 1 == 1, roots cannot be None
            let mut left_root =  pol.roots
                                            .pop()
                                            .unwrap();

            mem::swap(&mut left_root.l_niece, &mut node.l_niece);
            mem::swap(&mut left_root.r_niece, &mut node.r_niece);

            let n_hash = types::parent_hash(&left_root.data, &node.data);
                
            let new_node = PolNode::new (
                                                n_hash,
                                            Some(Box::new(left_root)),
                                            Some(Box::new(node))
                                                );

            return Pollard::create_root(pol, new_node, num_leaves >> 1);
        }

        node
    }
    // AddSingle adds a single given utxo to the tree
    // TODO activate caching (use remember). This isn't done in the
    // Go repo either yet
    fn add_single(&mut self, utxo: sha256::Hash, _remember: bool) {
        // init node. If the Pollard is perfect (meaning only one root), this will become a
        // new root
        let node = PolNode {
            data: utxo,
            l_niece: None,
            r_niece: None,
        };

        let add_node = Pollard::create_root(self, node, self.num_leaves);

        self.roots.push(add_node);

        // increment leaf count
        self.num_leaves += 1;
    }
    // Removes a set of UTXOS, given its position in the tree. If two siblings are deleted,
    // then we delete their aunt, if aut is a root, we keep the root, but nil out both siblings
    // and root's data.
    fn remove(&mut self, dels: Vec<u64>) -> Result <usize, String> {
        // if there is nothing to delete, return
        if dels.len() == 0 {
            return Ok(self.num_leaves as usize);
        }

        let leaves_after_del = self.num_leaves - dels.len() as u64;
        
        let mut dels = dels.to_owned();
        dels.sort();
        let dels = util::extract_twins(dels, util::tree_rows(self.num_leaves));
        println!("{:?}", dels);
        for i in dels.iter() {

            let (tree, node, bits) = util::detect_offset(*i, self.num_leaves).unwrap();

            // is that a root?
            if node == 0 {
                self.roots[tree as usize].l_niece = None;
                self.roots[tree as usize].r_niece = None;
                self.roots[tree as usize].data = bitcoin_hashes::Hash::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
            
            } else {
                //Get the node's aunt
                let aunt = Pollard::get_node_aunt(&mut self.roots[tree as usize], node, bits);
                
                if let Some(i) = aunt {
                    //If 0, then we are removing the left niece
                    if bits & 1 == 0 {
                        // If I'm removing a left niece, then the right niece gets promoted
                        // to it's aunt position
                        if let Some(r_niece) = &i.r_niece {
                            *i = r_niece.clone();
                        }                     
                    } else {
                        if let Some(l_niece) = &i.l_niece {
                            *i = l_niece.clone();
                        } 
                    }
                } else {
                    let node = &mut self.roots[tree as usize];
                    //If 0, then we are removing the left niece
                    if bits & 1 == 0 {
                        // If I'm removing a left niece, then the right niece gets promoted
                        // to it's aunt position
                        if let Some(r_niece) = &node.r_niece {
                            *node = *r_niece.clone();
                        }                     
                    } else {
                        if let Some(l_niece) = &node.l_niece {
                            *node = *l_niece.clone();
                        } 
                    }                       
                }

            }
        }

        
        Ok(leaves_after_del as usize)
    }

    // Get the aunt of a given node. If this node is a root, error if not found
    fn get_node_aunt(node: &mut PolNode, branch_len: u8, bits: u64) -> Option<&mut Box<PolNode>> {
        if branch_len == 1 {
            return None;
        }

        let mut len = branch_len - 1;
        
        let mut node = if (bits >> len) & 1 == 0 {
            node.l_niece.as_mut()
        } else {
            node.r_niece.as_mut()
        };

        while len > 1 {
            let lr = (bits >> branch_len) & 1;
            
            if let Some(i) = node {
                if lr == 0 {
                    node = i.l_niece.as_mut();                    
                } else {
                    node = i.r_niece.as_mut();                    
                }
            }
            
            len -= 1;
        }

        node
    }
}

/// PolNode represents a node in the utreexo pollard tree. It points
/// to its nieces
#[derive(Clone, Default, Debug)]
pub struct PolNode {
    // The hash
    data: sha256::Hash,
    l_niece: Option<Box<PolNode>>,
    r_niece: Option<Box<PolNode>>,
}
#[allow(dead_code)]
impl PolNode {
    /// aunt_op returns the hash of a nodes' nieces. Errors if called on nieces
    /// that are nil.
    fn aunt_op(&self) -> sha256::Hash {
        types::parent_hash(&self.l_niece.as_ref().unwrap().data, &self.r_niece.as_ref().unwrap().data)
    }
    /// Is this node a leaf?
    fn dead_end(&self) -> bool {
        self.l_niece.is_none() && self.r_niece.is_none()
    }
    /// Chop removes both nieces from a node, effectively deleting subtrees
    fn chop(&mut self) {
        self.l_niece = None;
        self.r_niece = None;
    }
    /// Return a new node with provided data. nieces may be None.
    fn new(data: sha256::Hash, l_niece: Option<Box<PolNode>>, r_niece: Option<Box<PolNode>>) -> PolNode {
        PolNode {
            data,
            l_niece,
            r_niece
        }
    }
    /// Prune a node by removing all its subtrees.
    fn prune(&mut self) {
        match &mut self.l_niece {
            None => (),
            Some(node) =>  {
                if node.dead_end() {
                    node.chop()
                }
            }
        }

        match &mut self.r_niece {
            None => (),
            Some(node) =>  {
                if node.dead_end() {
                    node.chop()
                }
            }
        }
    }
}

/// hashableNode is a node with all data that it's needed for hashing
pub struct HashableNode {
    pub sib: Option<Box<PolNode>>,
    pub dest: Option<Box<PolNode>>,
    pub position: u64 // doesn't really need to be there, but convenient for debugging
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::vec;

    use crate::accumulator::util;

    // A Utreexo tree will always have a collection of trees that are a perfect power
    // of two. The popcount of leaves should always equal the length of the root
    fn check_count(num_leaves: u64, root_len: usize) {
        let root_count = num_leaves.count_ones() as usize;
        assert_eq!(root_count, root_len);
    }
    #[test]
    fn get_line_count() {
        let (tree, depth, bits) = util::detect_offset(6, 4).unwrap();
        println!("{} {} {}", tree, depth, bits);
    }

    #[test]
    fn test_pol_del() {
        use bitcoin::hashes::{sha256, Hash, HashEngine};
        use super::types;

        let mut pol = super::Pollard::new();

        for i in 0..4 {
            let mut engine = bitcoin::hashes::sha256::Hash::engine();
            engine.input(&[i as u8]);
            let h = sha256::Hash::from_engine(engine);
            let leaf = types::Leaf{hash: h, remember: false};
            println!("{}", leaf.hash);
            // add one leaf
            if let Err(what) = pol.modify(vec![leaf], vec![]) {
                println!("{}", what);
                panic!();
            };
        }
        let v: Vec<_> = Vec::from([0, 3]);
        if let Err(what) = pol.modify(vec![], v) {
            println!("{}", what);
            panic!();
        }
        
        // let v: Vec<_> = Vec::from([500, 501, 502, 503, 504, 505, 506]);
        // if let Err(what) = pol.modify(vec![], v) {
        //     println!("{}", what);
        //     panic!();
        // }
        println!("{:#?}", pol);
    }

    #[test]
    fn test_pol_add() {
        use bitcoin::hashes::{sha256, Hash, HashEngine};
        use super::types;

        let mut pol = super::Pollard::new();

        for i in 0..6 {
            let mut engine = bitcoin::hashes::sha256::Hash::engine();
            engine.input(&[(i % 255) as u8]);
            let h = sha256::Hash::from_engine(engine);
            let leaf = types::Leaf{hash: h, remember: false};
            
            if let Err(what) = pol.modify(vec![leaf], vec![]) {
                println!("{}", what);
                panic!();
            }
        }
        assert!(pol.num_leaves == 6);

        // After an execution, check the number of Pollard's roots
        check_count(pol.num_leaves, pol.roots.len());
    }
}
