use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::bitcoin::secp256k1::rand;
use bdk_wallet::bitcoin::secp256k1::rand::RngCore;
use bdk_wallet::KeychainKind;
use bdk_wallet::template::{Bip86, DescriptorTemplate};

fn main() -> () {

    let mut seed: [u8; 32] = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    let network: Network = Network::Signet;
    let xprv: Xpriv = Xpriv::new_master(network, &seed).unwrap();
    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External).build(network).unwrap();
    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal).build(network).unwrap();

    println!(
        "----------------  Descriptor  ------------------------------\n{:?}\n{:?}\n",
        descriptor.to_string_with_secret(&key_map),
        descriptor.to_string()
    );
    println!(
        "----------------  Change Descriptor  -----------------------\n{:?}\n{:?}\n",
        change_descriptor.to_string_with_secret(&change_key_map),
        change_descriptor.to_string()
    );
}
