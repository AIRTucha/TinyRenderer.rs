use core::future::Future;
use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! run {
    ( $x:expr ) => {
        wasm_bindgen_futures::future_to_promise(async {
            unsafe {
                $x.await;
            }
            Ok(JsValue::UNDEFINED)
        });
    };
}
