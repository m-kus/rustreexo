// Rustreexo

use std::{collections::HashMap, task::Poll};
use std::mem;

use super::{
    types,
    util,
    transform
};

use bitcoin::hashes::{sha256, Hash, HashEngine};

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
    pub fn modify(&mut self, utxos: Vec<types::Leaf>, stxos: Vec<u64>) {
        // Order matters here. Adding then removing will result in a different
        // tree vs deleting then adding. For ease of use, only modify is visible
        // for external crates. This is consensus critical.
        Pollard::remove(self, stxos);
        Pollard::add(self, utxos);
    }

    pub fn add(&mut self, adds: Vec<types::Leaf>) {
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
                                            Some(Box::new(left_root.clone())),
                                            Some(Box::new(node.clone()))
                                                );
            left_root.aunt = Some(Box::from(new_node.clone()));
            node.aunt = Some(Box::from(new_node.clone()));

            return Pollard::create_root(pol, new_node, num_leaves >> 1);
        }

        node
    }
    // AddSingle adds a single given utxo to the tree
    // TODO activate caching (use remember). This isn't done in the
    // Go repo either yet
    fn add_single(&mut self, utxo: sha256::Hash, remember: bool) {
        // init node. If the Pollard is perfect (meaning only one root), this will become a
        // new root
        let node = PolNode {
            data: utxo,
            aunt: None,
            l_niece: None,
            r_niece: None,
        };

        let add_node = Pollard::create_root(self, node, self.num_leaves);

        self.roots.push(add_node);

        // increment leaf count
        self.num_leaves += 1;
    }

    fn remove(&mut self, dels: Vec<u64>) -> Result <usize, String> {
        // if there is nothing to delete, return
        if dels.len() == 0 {
            return Ok(self.num_leaves as usize);
        }

        let leaves_after_del = self.num_leaves - dels.len() as u64;

        for i in dels.iter() {
            let (tree, node, bits) = util::detect_offset(*i, self.num_leaves).unwrap();
            let n = Pollard::get_node(&mut self.roots[tree as usize], node, bits)?;
            if bits & 1 == 0 {
                n.l_niece = None;
            } else {
                n.r_niece = None;
            }
        }
        self.num_leaves = leaves_after_del;
        Ok(leaves_after_del as usize)
    }
    fn get_node(node: &mut PolNode, branch_len: u8, bits: u64) -> Result <&mut PolNode, String>{
        let mut len = branch_len - 1;
        if branch_len == 1 {
            return Ok(node);
        }

        let mut node = if (bits >> len) & 1 == 0 {
            node.l_niece.as_mut().unwrap()
        } else {
            node.r_niece.as_mut().unwrap()
        };
        while len > 1 {
            let lr = (bits >> branch_len) & 1;
            if lr == 0 {
                node = node.l_niece.as_mut().unwrap();
            } else {
                node = node.r_niece.as_mut().unwrap();
            }
            len -= 1;
        }
        Ok(node)
    }


}

/// PolNode represents a node in the utreexo pollard tree. It points
/// to its nieces
#[derive(Clone, Default, Debug)]
pub struct PolNode {
    // The hash
    pub data: sha256::Hash,
    pub aunt: Option<Box<PolNode>>,
    pub l_niece: Option<Box<PolNode>>,
    pub r_niece: Option<Box<PolNode>>,
}

impl PolNode {
    /// aunt_op returns the hash of a nodes' nieces. Errors if called on nieces
    /// that are nil.
    fn aunt_op(&self) -> sha256::Hash {
        types::parent_hash(&self.l_niece.as_ref().unwrap().data, &self.r_niece.as_ref().unwrap().data)
    }

    fn dead_end(&self) -> bool {
        self.l_niece.is_none() && self.r_niece.is_none()
    }

    fn chop(&mut self) {
        self.l_niece = None;
        self.r_niece = None;
    }
    fn new(data: sha256::Hash, l_niece: Option<Box<PolNode>>, r_niece: Option<Box<PolNode>>) -> PolNode {
        PolNode {
            data,
            aunt: None,
            l_niece,
            r_niece
        }
    }
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

// hashableNode is the data needed to perform a hash
pub struct HashableNode {
    pub sib: Option<Box<PolNode>>,
    pub dest: Option<Box<PolNode>>,
    pub position: u64 // doesn't really need to be there, but convenient for debugging
}

// polSwap swaps the contents of two polNodes & leaves pointers
fn pol_swap<'a, 'b>(mut a: &'a mut PolNode, mut asib: &'b mut PolNode, mut b: &'a mut PolNode, mut bsib: &'b mut PolNode) {
    mem::swap(&mut a, &mut b);
    mem::swap(&mut asib,&mut bsib);
}

#[cfg(test)]
mod tests {

    // A Utreexo tree will always have a collection of trees that are a perfect power
    // of two. The popcount of leaves should always equal the length of the root
    fn check_count(num_leaves: u64, root_len: usize) {
        let root_count = num_leaves.count_ones() as usize;
        assert_eq!(root_count, root_len);
    }

    fn check_root() {
    }
    // #[test]
    // fn test_pol_del() {
    //     use bitcoin::hashes::{sha256, Hash, HashEngine};
    //     use super::types;

    //     let mut pol = super::Pollard::new();

    //     for i in 0..5 {
    //         // boilerplate hashgen
    //         // TODO maybe there's a better way?
    //         let mut engine = bitcoin::hashes::sha256::Hash::engine();
    //         let num: &[u8; 1] = &[i as u8];
    //         engine.input(num);
    //         let h = sha256::Hash::from_engine(engine);
    //         println!("for i {}: {:?}", i, &h);
    //         let leaf = types::Leaf{hash: h, remember: false};

    //         // add one leaf
    //         pol.modify(vec![leaf], vec![]);
    //     }

    //     for i in 0..4 {
    //         let node = pol.grab_pos(i);
    //         match node {
    //             Err(e) => (panic!("no pollard node found")),
    //             Ok((node, node_sib, hn)) => {
    //                 let mut engine = bitcoin::hashes::sha256::Hash::engine();
    //                 let num: &[u8; 1] = &[i as u8];
    //                 engine.input(num);
    //                 let h = sha256::Hash::from_engine(engine);

    //                 println!("fetched node hash {}: {:?}", i, &node.l_niece.unwrap().data);
    //                 println!("fetched node_sib hash: {:?}", &node_sib.data);
    //                 println!("calculated 0 hash: {:?}", h);
    //             }
    //         }
    //     }
    //     let node = pol.grab_pos(14);

    //     match node {
    //         Err(e) => (panic!("no pollard node found")),
    //         Ok((node, node_sib, hn)) => {
    //             println!("fetched node hash {}: {:?}", 8, &node.l_niece.unwrap().data);
    //         }
    //     }


    // pol.modify(vec![], vec![0]);
    // }

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
            pol.modify(vec![leaf], vec![]);
        }
        assert!(pol.num_leaves == 6);

        // After an execution, check the number of Pollard's roots
        check_count(pol.num_leaves, pol.roots.len());
    }
}
