use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Response};

pub async fn get(url: &str) -> String {
    JsFuture::from(
        JsFuture::from(window().unwrap().fetch_with_str(url))
            .await
            .unwrap()
            .dyn_into::<Response>()
            .unwrap()
            .text()
            .unwrap(),
    )
    .await
    .unwrap()
    .as_string()
    .unwrap()
}
