#![no_std]
// #![cfg_attr(not(feature = "std"), no_std )]
extern crate alloc;
extern crate console_error_panic_hook;
use js_sys::*;
use wasm_bindgen::prelude::*;
use alloc::{format, vec};
use alloc::string::{ToString,String};
use md5::{Md5, Digest};
use wasm_bindgen::JsValue;
use alloc::vec::*;
use hex;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = wx)]
    fn showModal(param: &Object);
    #[wasm_bindgen(js_namespace = wx)]
    fn showToast(param: &JsValue);
}
#[wasm_bindgen]
pub fn greet(name: String) {
    log(&format!("hello {:?}.",name));
}

#[wasm_bindgen]
pub fn take_string_by_value(x: JsString) {
    log(&format!("take_string_by_value: {:?}",&x));
}

#[wasm_bindgen(js_name = md5calc)]
pub fn md5(inputval: JsValue) -> Result<JsValue, JsValue> {

    log(&format!("input data =  {:#?}", inputval) );
    // let data: String = JsString::from(inputval).into();
    let data: Vec<u8> = serde_wasm_bindgen::from_value(inputval)?;
    log(&format!("data =  {:?}", data));
    let mut hasher = Md5::new();
    hasher.update(data);
    let result: Vec<u8> = hasher.finalize().to_vec();
    log(&format!("md5 result =  {:?}\t{:?}", result, hex::encode(&result)));
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

// #[wasm_bindgen]
// pub fn md6(input: Uint8Array) -> Uint8Array {
//     // let mut hasher = Md5::new();
//     // hasher.update(input);
//     // let result = hasher.finalize();
//     // result.into_bytes().;
// }

/**
* 模块初始化时调用。
*/
#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    log("wasm running.");
    Ok(())
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test11() {
        assert_eq!(2+2, 4);
    }
}
