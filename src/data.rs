#[derive(Debug, Clone)]
pub struct Tx {
    pub tx_id: TxID,
    pub chain_code_name: Name,
    pub fn_name: Name,
    pub arguments: Arguments,
    pub signs: Signs,
    pub read_set: ReadSet,
    pub write_set: ReadSet,
}

pub type TxID = String;
pub type Name = String;
pub type Arguments = Vec<String>;
pub type Signs = Vec<Sign>;
pub type Sign = String;
pub type ReadSet = Vec<KVPair>;
pub type WriteSet = Vec<KVPair>;
pub type KVPair = [String; 2];
