use std::panic;

extern crate console_error_panic_hook;
// use concrete_core_wasm::{DefaultEngine, JsFunctionSeeder, LweDimension, Variance};
use concrete_core::prelude::*;
// use concrete_commons::parameters::LweDimension;
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
// pub fn run_rust() {
//     main();
// }

#[test]
fn main() {
    let lwe_dimension = LweDimension(750);
    let noise = Variance(2_f64.powf(-104.));

    // Here a hard-set encoding is applied on the input (shift by 59 bits) which corresponds here
    // to a precision of 4 bits with an additional bit of padding (won't be used but required for
    // PBS)
    let raw_input = 3_u64 << 59;

    // We will multiply by 4
    let raw_input_cleatext = 4_u64;

    // Unix seeder must be given a secret input.
    // Here we just give it 0, which is totally unsafe.
    const UNSAFE_SECRET: u128 = 0;
    let mut engine = DefaultEngine::new(Box::new(UnixSeeder::new(UNSAFE_SECRET))).unwrap();

    // We create a cleartext from the raw cleartext
    let cleartext: Cleartext64 = engine.create_cleartext(&raw_input_cleatext).unwrap();
    let key: LweSecretKey64 = engine.create_lwe_secret_key(lwe_dimension).unwrap();

    // We crate the input plaintext from the raw input
    let input_plaintext = engine.create_plaintext(&raw_input).unwrap();
    let input_ciphertext = engine
        .encrypt_lwe_ciphertext(&key, &input_plaintext, noise)
        .unwrap();

    // The content of the output ciphertext will be discarded, use a placeholder plaintext of 0
    let placeholder_output_plaintext = engine.create_plaintext(&0u64).unwrap();
    let mut ouptut_ciphertext = engine
        .encrypt_lwe_ciphertext(&key, &placeholder_output_plaintext, noise)
        .unwrap();

    // Perform the multiplication, overwriting (discarding) the output ciphertext content
    engine
        .discard_mul_lwe_ciphertext_cleartext(&mut ouptut_ciphertext, &input_ciphertext, &cleartext)
        .unwrap();

    // Get the decrypted result as a plaintext and then a raw value
    let decrypted_plaintext = engine
        .decrypt_lwe_ciphertext(&key, &ouptut_ciphertext)
        .unwrap();
    let raw_decrypted_plaintext = engine.retrieve_plaintext(&decrypted_plaintext).unwrap();

    // Round the output for our 4 bits of precision
    let output = raw_decrypted_plaintext >> 58;
    let carry = output % 2;
    let output = ((output >> 1) + carry) % (1 << 5);

    // Check the high bits have the result we expect
    assert_eq!(output, 12);

    engine.destroy(cleartext).unwrap();
    engine.destroy(key).unwrap();
    engine.destroy(input_plaintext).unwrap();
    engine.destroy(placeholder_output_plaintext).unwrap();
    engine.destroy(decrypted_plaintext).unwrap();
    engine.destroy(input_ciphertext).unwrap();
    engine.destroy(ouptut_ciphertext).unwrap();
}
