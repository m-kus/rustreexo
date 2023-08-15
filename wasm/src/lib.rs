use js_sys::Array;
use rustreexo::accumulator::{proof::Proof as RustProof, stump::Stump as RustStump};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

#[wasm_bindgen]
pub struct Stump(RustStump);
#[wasm_bindgen]
pub struct Proof(RustProof);
#[wasm_bindgen]
pub struct UpdateData(rustreexo::accumulator::stump::UpdateData);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl Stump {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(RustStump::new())
    }
    #[wasm_bindgen(js_name = "verify")]
    pub unsafe fn verify(&self, proof: Proof, del_hashes: Array) -> Result<bool, String> {
        let del_hashes = from_js_array(del_hashes);
        self.0.verify(&proof.0, &del_hashes)
    }
    #[wasm_bindgen(js_name = "modify")]
    pub unsafe fn modify_js(
        &mut self,
        utxos: js_sys::Array,
        stxos: Array,
        proof: Proof,
    ) -> Result<UpdateData, String> {
        let hashes: Vec<rustreexo::accumulator::node_hash::NodeHash> = from_js_array(utxos);
        let stxos = from_js_array(stxos);
        let (stump, update_data) = self.0.modify(&hashes, &stxos, &proof.0)?;
        self.0 = stump;
        Ok(UpdateData(update_data))
    }
    #[wasm_bindgen(getter, js_name = "roots")]
    pub unsafe fn roots_js(&self) -> js_sys::Array {
        let roots = &self.0.roots;
        into_js_array(roots)
    }
    #[wasm_bindgen(getter, js_name = "leaves")]
    pub unsafe fn leaves_js(&self) -> u64 {
        self.0.leaves
    }
    #[wasm_bindgen(setter, js_name = "roots")]
    pub unsafe fn set_roots_js(&mut self, roots: js_sys::Array) {
        self.0.roots = from_js_array(roots);
    }
    #[wasm_bindgen(setter, js_name = "leaves")]
    pub unsafe fn set_leaves_js(&mut self, leaves: u64) {
        self.0.leaves = leaves;
    }
}

#[wasm_bindgen]
impl Proof {
    #[wasm_bindgen(constructor)]
    /// A new empty proof.
    pub fn new() -> Proof {
        Self(RustProof::default())
    }
    #[wasm_bindgen]
    pub unsafe fn update(
        &mut self,
        cached_hashes: Array,
        add_hashes: Array,
        block_targets: Vec<u32>,
        remembers: Vec<u32>,
        update_data: UpdateData,
    ) -> Result<Array, String> {
        let cached_hashes = from_js_array(cached_hashes);
        let add_hashes = from_js_array(add_hashes);
        let update_data = update_data.0;
        let block_targets = block_targets.iter().map(|&x| x as u64).collect();
        let remembers = remembers.iter().map(|&x| x as u64).collect();
        let (proof, del_hashes) = self.0.clone().update(
            cached_hashes,
            add_hashes,
            block_targets,
            remembers,
            update_data,
        )?;
        self.0 = proof;
        Ok(into_js_array(&del_hashes))
    }
    /// Instantiates a new proof from a list of targets and hashes.
    #[wasm_bindgen(js_name = "from")]
    pub unsafe fn from(targets: &[u32], hashes: js_sys::Array) -> Result<Proof, String> {
        let hashes = from_js_array(hashes);
        let proof = RustProof {
            targets: targets.iter().map(|&x| x as u64).collect(),
            hashes,
        };
        Ok(Proof(proof))
    }
    #[wasm_bindgen(getter, js_name = "targets")]
    pub unsafe fn targets_js(&self) -> Vec<u64> {
        self.0.targets.clone()
    }
    #[wasm_bindgen(getter, js_name = "hashes")]
    pub unsafe fn hashes_js(&self) -> js_sys::Array {
        let hashes = &self.0.hashes;
        into_js_array(hashes)
    }
    #[wasm_bindgen(setter, js_name = "targets")]
    pub unsafe fn set_targets_js(&mut self, targets: &[u32]) {
        self.0.targets = targets.iter().map(|&x| x as u64).collect();
    }
    #[wasm_bindgen(setter, js_name = "hashes")]
    pub unsafe fn set_hashes_js(&mut self, hashes: js_sys::Array) {
        self.0.hashes = from_js_array(hashes);
    }
}
unsafe fn from_js_array<T: From<Vec<u8>>>(js_array: js_sys::Array) -> Vec<T> {
    let mut vec = Vec::new();
    for value in js_array {
        if let Ok(value) = value.dyn_into::<js_sys::Uint8Array>() {
            let el = value.to_vec();
            vec.push(T::from(el));
        }
    }
    vec
}
unsafe fn into_js_array<T: Into<Vec<u8>> + Copy>(vec: &Vec<T>) -> js_sys::Array {
    let array = js_sys::Array::new();
    for &el in vec {
        let el: Vec<u8> = el.into();
        let el = js_sys::Uint8Array::from(el.as_slice());
        array.push(&el.into());
    }
    array
}

#[wasm_bindgen]
pub struct LeafData {
    #[wasm_bindgen(skip)]
    pub block_hash: [u8; 32],
    #[wasm_bindgen(skip)]
    pub tx_id: [u8; 32],
    pub outpoint: u64,
    pub amount: u64,
    #[wasm_bindgen(skip)]
    pub spk: Vec<u8>,
}

#[wasm_bindgen]
impl LeafData {
    #[wasm_bindgen(getter, js_name = "leaf_hash")]
    pub fn leaf_hash_js(&self) -> js_sys::Uint8Array {
        todo!()
    }
    #[wasm_bindgen(constructor)]
    pub fn new(
        block_hash: js_sys::Uint8Array,
        tx_id: js_sys::Uint8Array,
        outpoint: u32,
        amount: u32,
        spk: js_sys::Uint8Array,
    ) -> Self {
        let block_hash = block_hash.to_vec();
        let tx_id = tx_id.to_vec();
        let spk = spk.to_vec();
        Self {
            block_hash: block_hash.try_into().unwrap(),
            tx_id: tx_id.try_into().unwrap(),
            outpoint: outpoint as u64,
            amount: amount as u64,
            spk,
        }
    }
    #[wasm_bindgen(setter, js_name = "block_hash")]
    pub fn set_block_hash_js(&mut self, block_hash: js_sys::Uint8Array) {
        self.block_hash = block_hash.to_vec().try_into().unwrap();
    }
    #[wasm_bindgen(setter, js_name = "spk")]
    pub fn set_spk_js(&mut self, spk: js_sys::Uint8Array) {
        self.spk = spk.to_vec();
    }
}
