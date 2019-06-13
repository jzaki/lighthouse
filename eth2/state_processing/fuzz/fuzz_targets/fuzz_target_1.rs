#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate ssz;
extern crate state_processing;
extern crate types;

use ssz::{Decode, DecodeError};
use types::*;
use state_processing::{per_block_processing, block_processing_builder::BlockProcessingBuilder};


// Fuzz per_block_processing, with input data as an ssz encoded block
fuzz_target!(|data: &[u8]| {
    // Generate a chain_spec
    let spec = FoundationEthSpec::spec();

    // Generate a BeaconState after genesis
    let builder = get_builder(&spec);
    let (_, mut state) = builder.build(None, None, &spec);

    // Convert data to a BeaconBlock
    let block = BeaconBlock::from_ssz_bytes(&data) ;

    // Fuzz per_block_processing (if decoding was successful)
    if !block.is_err() {
        per_block_processing(&mut state, &block.unwrap(), &spec);
    }
});

fn get_builder(spec: &ChainSpec) -> (BlockProcessingBuilder<FoundationEthSpec>) {
    let num_validators = 2;
    let mut builder = BlockProcessingBuilder::new(num_validators, &spec);

    // Set the state and block to be in the last slot of the 4th epoch.
    let last_slot_of_epoch = (spec.genesis_epoch + 4).end_slot(spec.slots_per_epoch);
    builder.set_slot(last_slot_of_epoch, &spec);
    builder.build_caches(&spec);

    (builder)
}
