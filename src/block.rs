extern crate byteorder;
extern crate itertools;
extern crate sha3;

use std::fmt;
use std::io::Write;

use self::byteorder::{LittleEndian, WriteBytesExt};
use self::itertools::Itertools;
use self::sha3::{Digest, Sha3_512};


pub const BLOCK_SIZE: usize = 1024;// * 1024 * 4;  // 4MB
pub type BlockData = [u8; BLOCK_SIZE];


pub const HASH_SIZE: usize = 64;
pub type Hash = [u8; HASH_SIZE];


#[derive(Clone)]
pub struct Block {
    index: u64,
    previous_hash: Hash,
    timestamp: u64,
    data: BlockData,
    hash: Hash
}

impl Block {
    pub fn from_slice(index: u64,
                      previous_hash: Hash,
                      timestamp: u64,
                      data: &[u8]) -> Block
    {
        let block_data = block_data_from_slice(data);
        let hash = compute_hash(index, previous_hash, timestamp, block_data);
        Block::new(index, previous_hash, timestamp, block_data, hash)
    }

    pub fn new(index: u64,
               previous_hash: Hash,
               timestamp: u64,
               data: BlockData,
               hash: Hash) -> Block
    {
        Block {
            index: index,
            previous_hash: previous_hash,
            timestamp: timestamp,
            data: data,
            hash: hash
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block(\nindex={}\ntimestamp={}\nprevious_hash={:02x}\ndata={:02x}\nhash={:02x}\n)",
            self.index,
            self.timestamp,
            self.previous_hash.iter().format(""),
            self.data.iter().format(""),
            self.hash.iter().format(""))
    }
}


pub fn block_data_from_slice(data: &[u8]) -> BlockData {
    assert!(
        data.len() <= BLOCK_SIZE,
        "Got {} bytes of data, but maximum allowed bytes is {}",
        data.len(),
        BLOCK_SIZE);
    let mut block_data: BlockData = [0; BLOCK_SIZE];
    for i in 0..data.len() {
        block_data[i] = data[i];
    }
    block_data
}


pub fn compute_block_hash(block: Block) -> Hash {
    compute_hash(block.index, block.previous_hash, block.timestamp, block.data)
}


fn compute_hash(index: u64, previous_hash: Hash, timestamp: u64, data: BlockData) -> Hash {
    let mut bytes_to_hash = vec![];
    bytes_to_hash.write_u64::<LittleEndian>(index).unwrap();
    bytes_to_hash.write(&previous_hash).unwrap();
    bytes_to_hash.write_u64::<LittleEndian>(timestamp).unwrap();
    bytes_to_hash.write(&data).unwrap();

    let mut hasher = Sha3_512::default();
    hasher.input(bytes_to_hash.as_slice());

    let mut hash: Hash = [0; HASH_SIZE];
    hash.copy_from_slice(hasher.result().as_slice());
    hash
}
