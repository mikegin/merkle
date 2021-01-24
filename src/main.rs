use std::hash::{Hash};
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use generic_array::{GenericArray};
use hex;

type GenArray = GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>;

#[derive(Debug, Copy, Clone, Hash, Serialize)]
struct Data {
    id: u32,
    nonce: u32
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}

fn hash_data<T: Hash + Serialize>(data: T) -> GenArray {
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

    let hash_array: Vec<GenArray> = clone.into_iter().map(|v| hash_data(v)).collect();

    let hex_array: Vec<String> = hash_array.clone().into_iter().map(|v| hex::encode(v)).collect();
    println!("hex_array {:?}", hex_array);

    //algo
    //if 0 elements, do nothing
    //if 2 elements, hash each, then combine hashes and hash again
    //if 4 elements, hash each, for each two, combine hashes and hash again -> vector (need to run through algo again)

    let mut new_hash_array = hash_array.clone();
    while new_hash_array.len() != 2 {

        let mut new = vec!();
        for i in (0..new_hash_array.len()).step_by(2) {
            let first = new_hash_array[i];
            let second = new_hash_array[i + 1];
            let combined = [first, second].concat();
            let combined_hash = hash_data(combined);
            new.push(combined_hash)
        }

        let next: Vec<String> = new.clone().into_iter().map(|v| hex::encode(v)).collect();
        println!("next {:?}", next);

        new_hash_array = new;
    }

    

    let first = new_hash_array[0];
    let second = new_hash_array[1];
    let combined = [first, second].concat();
    let combined_hash = hash_data(combined);

    println!("combined_hash {:?}", hex::encode(combined_hash));

}

fn test1() {
    println!("<<<<<<<<<<<<<TEST1>>>>>>>>>>>>");

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

fn test2() {
    println!("<<<<<<<<<<<<<TEST2>>>>>>>>>>>>");

    let data1 = Data {
        id: 1,
        nonce: 0
    };

    let vector = vec!(data1);

    create_merkle(vector);
}

fn test3() {
    println!("<<<<<<<<<<<<<TEST3>>>>>>>>>>>>");

    let data1 = Data {
        id: 1,
        nonce: 0
    };

    let data2 = Data {
        id: 2,
        nonce: 0
    };

    let data3 = Data {
        id: 3,
        nonce: 0
    };

    let vector = vec!(data1, data2, data3);

    create_merkle(vector);
}

fn test4() {
    println!("<<<<<<<<<<<<<TEST4>>>>>>>>>>>>");

    let data1 = Data {
        id: 1,
        nonce: 0
    };

    let data2 = Data {
        id: 2,
        nonce: 0
    };

    let data3 = Data {
        id: 3,
        nonce: 0
    };

    let data4 = Data {
        id: 4,
        nonce: 0
    };

    let vector = vec!(data1, data2, data3, data4);

    create_merkle(vector);
}
