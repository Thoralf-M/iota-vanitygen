use anyhow::Result;
use bee_message::prelude::{Address, Ed25519Address};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Curve, Seed},
};
use std::convert::TryInto;

pub fn generate_address(
    seed: &Seed,
    account_index: u32,
    address_index: u32,
    internal: bool,
) -> Result<Address> {
    // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    let chain = Chain::from_u32_hardened(vec![
        44,
        4218,
        account_index,
        internal as u32,
        address_index,
    ]);
    let public_key = seed
        .derive(Curve::Ed25519, &chain)?
        .secret_key()
        .public_key()
        .to_bytes();
    // Hash the public key to get the address
    let result = Blake2b256::digest(&public_key).try_into().map_err(|_e| {
        anyhow::anyhow!("Hashing the public key while generating the address failed.")
    });

    Ok(Address::Ed25519(Ed25519Address::new(result?)))
}
