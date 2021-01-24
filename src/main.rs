use std::hash::{Hash};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use generic_array::{GenericArray};
use hex;

#[derive(Debug, Copy, Clone, Hash, Serialize)]
struct Data {
    id: u32,
    nonce: u32
}

fn main() {
    let data1 = Data {
        id: 1,
        nonce: 0
    };

    let data2 = Data {
        id: 2,
        nonce: 0
    };

    let vector = vec!(data1, data2);

    create_merkle(vector);
}

fn hash_data<T: Hash + Serialize>(data: T) -> GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>> {
    let mut hasher = Sha256::new();
    let bytes = bincode::serialize(&data).unwrap();
    hasher.update(bytes);
    let result = hasher.finalize();
    result
}

fn create_merkle<T: Hash + Serialize + Clone + Copy>(data: Vec<T>) {
    let mut clone = data.clone();
    if data.len() % 2 != 0 {
        let last_element = clone[(data.len() - 1)];
        clone.push(last_element)
    }

    //algo
    //if 0 elements, do nothing
    //if 2 elements, hash each, then combine hashes and hash again
    //if 4 elements, hash each, for each two, combine hashes and hash again -> vector (need to run through algo again)

    if clone.len() != 2 {
        let mut clone2 = clone.clone();
        while clone2.len() != 2 {
            let mut clone3 = vec!();
            for i in (0..clone2.len()).step_by(2) {
                // Do stuff
            }
        }
    }

    let first = clone[0];
    let second = clone[1];
    let first_hash = hash_data(first);
    let second_hash = hash_data(second);
    let combined = [first_hash, second_hash].concat();
    let combined_hash = hash_data(combined);

    println!("first_hash {:?}", hex::encode(first_hash));
    println!("second_hash {:?}", hex::encode(second_hash));
    println!("combined_hash {:?}", hex::encode(combined_hash));

}
