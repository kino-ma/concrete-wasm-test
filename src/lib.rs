use std::panic;

extern crate console_error_panic_hook;
use concrete_csprng::generators::{RandomGenerator, SoftwareRandomGenerator};
use concrete_csprng::seeders::{JsSeeder, Seeder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn generate() {
    log("enter");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("set hook");
    let mut seeder = JsSeeder;
    log("seeder");
    let seed = seeder.seed();
    log("seed");
    let mut generator = SoftwareRandomGenerator::new(seed);
    log("generator");
    let number = generator.next().unwrap();
    log("generate");
    alert(&format!("generated: {}", number));
    log("return");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
