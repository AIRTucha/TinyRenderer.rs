use core::option::Option;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};

use web_sys::console;
/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[wasm_bindgen]
pub async fn get() {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master",
        &opts,
    )
    .unwrap();

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json");

    let window = web_sys::window().unwrap();
    let result = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request));
    let r = result.await.unwrap();
    console::log_1(&r);
}
