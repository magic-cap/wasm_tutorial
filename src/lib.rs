use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader};
use magic_cap;
use std::io::Cursor;
use js_sys::{JsString, Object, Uint8Array};

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
async fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    use web_sys::console;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    //let url = format!("https://github.com/magic-cap/magic-cap/raw/refs/heads/main/kitten.mcap");
    let url = format!("kitten.mcap");

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/binary")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    console::log_1(&"about to do future stuff".into());

    // Convert this other `Promise` into a rust `Future`.
    let stream = resp.body().ok_or("no kittens")?;
    let reader: ReadableStreamDefaultReader = stream.get_reader().dyn_into().expect("reader failed");


    let mut data: Vec<u8> = vec![];

    let mut result_value = JsFuture::from(reader.read()).await;
    let mut done = false;
    while !done {
        if let Ok(stuff) = result_value {
            console::log_1(&"trying to load a chunk".into());
            let result: Object = stuff.dyn_into().unwrap();
            console::log_1(&format!("result is {:?}", result).into());
            let chunk_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
            // next line fails when we're done
            let chunk_array: Uint8Array = chunk_value.dyn_into().unwrap();
            let mut chunk = chunk_array.to_vec();
            console::log_1(&format!("got some {}", chunk.len()).into());
            data.append(&mut chunk);
            result_value = JsFuture::from(reader.read()).await;
        } else {
            done = true;
        }
    }

    console::log_1(&"loading mcap".into());
    console::log_1(&format!("chunk size: {}", data.len()).into());

    // this just "unreachables" into the js console -- can we error-handle better?
    let mc = match magic_cap::Immutable::read(Cursor::new(data)) {
        Ok(x) => {"loaded successfully"}
        Err(e) => {"error loading mcap"}
    };

    let foo = format!("mcap: {:?}", mc);
    let val = document.create_element("p")?;
    val.set_inner_html(foo.as_str());
    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
