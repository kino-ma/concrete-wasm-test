use std::panic;

extern crate console_error_panic_hook;
use concrete::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() {
    main().expect("failed to run concrete");
}

fn main() -> Result<(), CryptoAPIError> {
    // generate a secret key
    let secret_key = LWESecretKey::new(&LWE128_630);

    // the two values to add
    let m1 = 8.2;
    let m2 = 5.6;

    // specify the range and precision to encode messages into plaintexts
    // here we encode in [0, 10[ with 8 bits of precision and 1 bit of padding
    let encoder = Encoder::new(0., 10., 8, 1)?;

    // encode the messages into plaintexts
    let p1 = encoder.encode_single(m1)?;
    let p2 = encoder.encode_single(m2)?;

    // encrypt plaintexts
    let mut c1 = VectorLWE::encrypt(&secret_key, &p1)?;
    let c2 = VectorLWE::encrypt(&secret_key, &p2)?;

    // add the two ciphertexts homomorphically
    c1.add_with_padding_inplace(&c2)?;

    // decrypt and decode the result
    let m3 = c1.decrypt_decode(&secret_key)?;

    // print the result and compare to non-FHE addition
    log(&format!("Real: {} + {} = {}", m1, m2, m1 + m2));
    log(&format!("FHE: {} + {} = {}", p1.decode()?[0], p2.decode()?[0], m3[0]));
    Ok(())
}
