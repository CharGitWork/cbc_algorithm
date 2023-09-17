#![allow(non_snake_case)]
#![allow(unused)]

use crate::encrypt_lib::cryptography::{BLOCK_SIZE, Encrypted};

use std::{
    env, process,
    collections::{
        HashMap,
        VecDeque
    }
};

pub struct Cli {
    pub plain_text: String,
}

pub trait CliTrait {
    fn create(Arg: env::Args) -> Cli;
    fn calling_root(Arg: Self) -> RootTx;
}

impl CliTrait for Cli {

    fn create(mut iterator: env::Args) -> Cli {

        iterator.next();

        if iterator.len() != 1 {
            eprintln!("INVALID_LENGHT");
            eprintln!("command: cargo run <MESSAGE>");
            process::exit(404);
        }

        let plain_text: String = iterator.next().unwrap();

        Cli { plain_text }
    }

    fn calling_root(cli: Self) -> RootTx {
        let bytes_vers = vec![];
        let binary_vers = vec![];
        let blocks = HashMap::new();
        
        RootTx {
            plain_text: cli.plain_text,
            bytes_vers,
            binary_vers,
            blocks,
        }
    }
}

pub struct RootTx {
    pub plain_text: String,
    pub bytes_vers: Vec<u8>, // temp pub
    pub binary_vers: Vec<u8>, // temp pub
    blocks: HashMap<usize, Block>
}

pub trait RootTrait {
    fn string_to_bytes(Arg: &mut RootTx);
    fn bytes_to_bits(Arg: &mut RootTx);
    fn separate_block(Arg: &mut RootTx);
}

impl RootTrait for RootTx {

    // @explain convert a user entry in Vec<u8> of bytes
    fn string_to_bytes(root: &mut RootTx) {

        let convert = &root.plain_text;
        let convert_lenght = convert.len();

        // @explain optimized for latin characters, they are encoded on a single byte 1 * 8 -> len * 8
        let mut bytes_vers: Vec<u8> = Vec::with_capacity(convert_lenght * 8);
        
            for byte in convert.as_bytes() {
                bytes_vers.push(*byte);
            }

        root.bytes_vers = bytes_vers;
    }

    // @explain convert a Vec<u8> of bytes into a binary Vec<u8>
    fn bytes_to_bits(root: &mut RootTx) {

        let bytes = &root.bytes_vers;
        let mut temp = Vec::with_capacity(bytes.len());
        bytes
            .iter()
            .for_each(|byte| {
                let arr = [0u8; 8];
                let len = arr.len();
                temp.push(from_byte_to_bits(*byte, arr, len - 1));
            });

        root.binary_vers = temp.into_iter().flatten().collect::<Vec<u8>>();
    }

    // @explain Creates a Map that stores the 128-bit blocks resulting from the separation of the binary sequence
    // obtained using the `bytes_to_bits` function
    fn separate_block(root: &mut RootTx) {

        let mut root = root;
        let binary_chain = &*root.binary_vers;
        let bits_len = root.binary_vers.len();
        let mut data_loss = 0u8;

        // @explain how many blocks is needed ..? N_blocks
        let N_blocks = if bits_len % BLOCK_SIZE == 0 {
            bits_len / BLOCK_SIZE
        } else {
            data_loss = (bits_len % BLOCK_SIZE) as u8;
            (bits_len / BLOCK_SIZE) + 1
        };

        let mut iStart = 0usize;

        // @explain iterate N time to create N block to store `bits_len` bits
        for id in 1..=N_blocks {
            let mut block: VecDeque<u8> = VecDeque::with_capacity(128);
            
                loop {
                    // @explain this block if is execute when the last block is "not full"
                    if data_loss != 0 && id == N_blocks {
                        for i in 0..BLOCK_SIZE as usize {

                            match binary_chain.get(iStart) {
                                Some(bit) => { block.push_back(*bit); 
                                    iStart += 1;
                                },
                                None => block.push_front(0),
                            }
                        }
                        break
                    } // @explain add all bits for each fully block, id == number of block
                    else if iStart < (BLOCK_SIZE * id) {
                        block.push_back(binary_chain[iStart]);
                        iStart += 1;
                    }
                    else {
                        break
                    }
                } 
            
            // @explain create a Block and add it in the RootTx
            let block = <Block as BlockTrait>::new_block(block, id);
            root.blocks.insert(id, block);
        }
    }
}

// @explain receiv a byte and convert it in a binary array
fn from_byte_to_bits(byte: u8, buffer: [u8; 8], iter: usize) -> [u8; 8] {

    if byte == 0 {
        return buffer
    }
    else if byte == 1 {
        let mut arr = buffer;
        arr[iter] = 1;
        return arr
    }
    else {
        let reste = byte % 2;
        let dividende = byte / 2;

        let mut arr = from_byte_to_bits(dividende, buffer, iter - 1);
        arr[iter] = reste;
        arr
    }  
}

struct Block {
    id: usize,
    store_block: VecDeque<u8>,
}

trait BlockTrait {
    fn new_block(Arg: VecDeque<u8>, id: usize) -> Block;
}

impl BlockTrait for Block {
    fn new_block(block: VecDeque<u8>, id: usize) -> Block {
        
        Block {
            id,
            store_block: block,
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_in_3_block_with_third_block_not_full() {
        let mut vec = vec![1u8; 128];
        vec.extend([0u8; 128]);
        // @explain only 48 bits
        vec.extend([1u8; 48]);

        let mut root = RootTx {
            plain_text: "hello".to_string(),
            bytes_vers: vec![],
            binary_vers: vec,
            blocks: HashMap::new()
        };
        <RootTx as RootTrait>::separate_block(&mut root);

        let first_block = root.blocks.get(&1).unwrap();
        let mut vecdeque = VecDeque::with_capacity(128);
        vecdeque.extend([1u8; 128]);
        assert_eq!(
            first_block.store_block,
            vecdeque
        );

        let second_block = root.blocks.get(&2).unwrap();
        let mut vecdeque = VecDeque::with_capacity(128);
        vecdeque.extend([0u8; 128]);
        assert_eq!(
            second_block.store_block,
            vecdeque
        );

        // @explain here the thrid block store the 48 bits to the right of this block
        let third_block = root.blocks.get(&3).unwrap();
        let mut vecdeque = VecDeque::with_capacity(128);
        vecdeque.extend([0u8; 80]);
        vecdeque.extend([1u8; 48]);
        assert_eq!(
            third_block.store_block,
            vecdeque
        );
    }

    #[test]
    fn convert_a_string_into_bytes() {
        let cli = Cli { plain_text: "hello".to_string() };
        let mut rootTx = CliTrait::calling_root(cli);

        assert!(rootTx.bytes_vers.is_empty());

        <RootTx as RootTrait>::string_to_bytes(&mut rootTx);

        assert_eq!(
            rootTx.bytes_vers,
            vec![104, 101, 108, 108, 111],
            "<BUG> `string_to_bytes()` <BUG>"
        );
    }

    #[test]
    fn convert_bytes_to_bits_function() {
        let cli = Cli { plain_text: "hello".to_string() };
        let mut rootTx = CliTrait::calling_root(cli);

        <RootTx as RootTrait>::string_to_bytes(&mut rootTx);

        <RootTx as RootTrait>::bytes_to_bits(&mut rootTx);

        assert_eq!(
            rootTx.binary_vers,
            vec![0, 1, 1, 0, 1, 0, 0, 0,   0, 1, 1, 0, 0, 1, 0, 1,   0, 1, 1, 0, 1, 1, 0, 0,   0, 1, 1, 0, 1, 1, 0, 0,   0, 1, 1, 0, 1, 1, 1, 1],
            "<BUG> `bytes_to_bits()` <BUG>"
        );
    }

    #[test]
    fn from_byte_to_bits_works() {

        let byte = 3;
        let buffer = [0u8; 8];

        let arr = from_byte_to_bits(byte, buffer, 7); // @explain 0..7 -> arr.len() == 8

        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 1, 1], "<BUG> `from_byte_to_bits()` <BUG>");
    }
}