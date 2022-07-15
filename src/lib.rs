use std::panic;

extern crate console_error_panic_hook;
use concrete_core_wasm::{DefaultEngine, JsFunctionSeeder, LweDimension, Variance};
// use concrete_commons::parameters::LweDimension;
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_rust() {
    main();
}

fn main() -> Result<(), JsError> {
    // // generate a secret key
    // let secret_key = LWESecretKey::new(&LWE128_630);

    // // the two values to add
    // let m1 = 8.2;
    // let m2 = 5.6;

    // // specify the range and precision to encode messages into plaintexts
    // // here we encode in [0, 10[ with 8 bits of precision and 1 bit of padding
    // let encoder = Encoder::new(0., 10., 8, 1)?;

    // // encode the messages into plaintexts
    // let p1 = encoder.encode_single(m1)?;
    // let p2 = encoder.encode_single(m2)?;

    // // encrypt plaintexts
    // let mut c1 = VectorLWE::encrypt(&secret_key, &p1)?;
    // let c2 = VectorLWE::encrypt(&secret_key, &p2)?;

    // // add the two ciphertexts homomorphically
    // c1.add_with_padding_inplace(&c2)?;

    // // decrypt and decode the result
    // let m3 = c1.decrypt_decode(&secret_key)?;

    // // print the result and compare to non-FHE addition
    // log(&format!("Real: {} + {} = {}", m1, m2, m1 + m2));
    // log(&format!(
    //     "FHE: {} + {} = {}",
    //     p1.decode()?[0],
    //     p2.decode()?[0],
    //     m3[0]
    // ));
    // Ok(())

    log("hello, world!");

    let lwe_dimension = LweDimension::new(750);
    let noise = Variance::new(2_f64.powf(-104.));

    // Here a hard-set encoding is applied on the input (shift by 59 bits) which corresponds here
    // to a precision of 4 bits with an additional bit of padding (won't be used but required for
    // PBS)
    // かける数
    let raw_input = 3_u32 << 27;

    // We will multiply by 4
    // かけられる数
    let raw_input_cleatext = 4_u32;

    let js_func = Function::new_no_args(
        r#"
        const array = new Uint8Array(16);
        return array
    "#,
    );
    let mut engine = DefaultEngine::new(JsFunctionSeeder::new(js_func))?;

    let clear_text = engine.create_cleartext_f64(12.3);
    let key = engine.create_lwe_secret_key_32(lwe_dimension)?;

    // We crate the input plaintext from the raw input
    let input_plaintext = engine.create_plaintext_32(raw_input)?;
    let input_ciphertext = engine.encrypt_lwe_ciphertext_32(&key, &input_plaintext, noise)?;

    let noise = Variance::new(2_f64.powf(-104.));
    // The content of the output ciphertext will be discarded, use a placeholder plaintext of 0
    let placeholder_output_plaintext = engine.create_plaintext_32(0u32)?;
    let mut ouptut_ciphertext =
        engine.encrypt_lwe_ciphertext_32(&key, &placeholder_output_plaintext, noise)?;

    // // Perform the multiplication, overwriting (discarding) the output ciphertext content
    // engine.discard_mul_lwe_ciphertext_cleartext(
    //     &mut ouptut_ciphertext,
    //     &input_ciphertext,
    //     &cleartext,
    // )?;

    // Get the decrypted result as a plaintext and then a raw value
    let decrypted_plaintext = engine.decrypt_lwe_ciphertext_32(&key, &ouptut_ciphertext)?;
    let raw_decrypted_plaintext = engine.retrieve_plaintext_32(&decrypted_plaintext)?;

    // Round the output for our 4 bits of precision
    let output = raw_decrypted_plaintext >> 26;
    let carry = output % 2;
    println!("output: {}, carry: {}", output, carry);
    let output = ((output >> 1) + carry) % (1 << 5);

    // Check the high bits have the result we expect
    assert_eq!(output, 12);

    // engine.destroy(cleartext)?;
    // engine.destroy(key)?;
    // engine.destroy(input_plaintext)?;
    // engine.destroy(placeholder_output_plaintext)?;
    // engine.destroy(decrypted_plaintext)?;
    // engine.destroy(input_ciphertext)?;
    // engine.destroy(ouptut_ciphertext)?;

    Ok(())
}
