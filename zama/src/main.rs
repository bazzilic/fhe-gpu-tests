use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use concrete::prelude::*;
use std::time::Instant;

fn main() {
    println!("Starting configiuration...");
    let before = Instant::now();
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();
    println!("Configuration done in {:?}", before.elapsed());

    let before = Instant::now();
    let (client_key, server_key) = generate_keys(config);
    println!("Key generation: {:?}", before.elapsed());

    set_server_key(server_key);

    let clear_a = 27u8;
    let clear_b = 128u8;

    let before = Instant::now();
    let a = FheUint8::encrypt(clear_a, &client_key);
    println!("Encrypting a = {}: {:?}", clear_a, before.elapsed());

    let before = Instant::now();
    let b = FheUint8::encrypt(clear_b, &client_key);
    println!("Encrypting b = {}: {:?}", clear_b, before.elapsed());

    let before = Instant::now();
    let result = a + b;
    println!("Addition: {:?}", before.elapsed());

    let before = Instant::now();
    let decrypted_result: u8 = result.decrypt(&client_key);
    println!("Decryption: {:?}", before.elapsed());

    let clear_result = clear_a + clear_b;

    assert_eq!(decrypted_result, clear_result);
}