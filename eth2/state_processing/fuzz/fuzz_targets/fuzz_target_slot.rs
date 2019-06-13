#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate ssz;
extern crate state_processing;
extern crate types;
extern crate hex;

use ssz::{Decode, DecodeError, Encode};
use types::*;
use state_processing::{per_block_processing, SPEC, BUILDER};


// Fuzz per_block_processing, the input is a default block varying the slot
//
// Note: There is no need to run with fuzz arguement max_len >= 8
fuzz_target!(|data: &[u8]| {
    // Generate a chain_spec
    let (mut block, mut state) = BUILDER.clone().build(None, None, &*SPEC);
    println!("Here");

    // Convert the data into a u64
    let mut slot = [0u8; 8];
    for (i, item) in data.iter().enumerate() {
        if i >= 8 {
            // Stop writing after 8
            break;
        }
        slot[i] = *item;
    }
    let slot = u64::from_be_bytes(slot);

    // Edit block slot to equal data
    block.slot = Slot::new(slot);
    // Optional: Also set state.slot = slot here to test different functionality.

    // Fuzz per_block_processing (if decoding was successful)
    per_block_processing(&mut state, &block, &*SPEC);
});
