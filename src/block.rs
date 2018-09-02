extern crate byteorder;
extern crate sha3;

use std::io::Write;
use self::byteorder::{LittleEndian, WriteBytesExt};
use self::sha3::{Digest, Sha3_512};


const BLOCK_SIZE: usize = 1024 * 1024 * 4;  // 4MB
type BlockData = [u8; BLOCK_SIZE];


const HASH_SIZE: usize = 512;
type Hash = [u8; HASH_SIZE];


struct Block {
    index: u64,
    previous_hash: Hash,
    timestamp: u64,
    data: BlockData,
    hash: Hash
}

impl Block {
    pub fn from_vec(index: u64,
                    previous_hash: Hash,
                    timestamp: u64,
                    data: Vec<u8>) -> Block
    {
        // TODO: can this copy be avoided? maybe, but doesn't matter, since
        // copy will be needed anyway?
        assert!(
            data.len() <= BLOCK_SIZE,
            "Got {} bytes of data, but maximum data size is {}",
            data.len(),
            BLOCK_SIZE);
        let mut data_buffer: BlockData = [0; BLOCK_SIZE];
        data_buffer.copy_from_slice(data.as_slice());

        let mut bytes_to_hash = vec![];
        bytes_to_hash.write_u64::<LittleEndian>(index).unwrap();
        bytes_to_hash.write(&previous_hash).unwrap();
        bytes_to_hash.write_u64::<LittleEndian>(timestamp).unwrap();
        bytes_to_hash.write(&data_buffer).unwrap();

        let mut hash: Hash = [0; HASH_SIZE];
        hash_sha3_512(bytes_to_hash.as_slice(), &mut hash);

        Block::new(index, previous_hash, timestamp, data_buffer, hash)
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


fn hash_sha3_512(data: &[u8], output: &mut Hash) {
    let mut hasher = Sha3_512::default();
    hasher.input(data);
    output.clone_from_slice(hasher.result().as_slice());
}
