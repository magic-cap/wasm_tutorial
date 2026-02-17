use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use magic_cap;
use std::io::Cursor;

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
    let jsvalue = JsFuture::from(resp.text()?).await?;
    let string = jsvalue.as_string().ok_or("it's not a string i guess")?;
    let text = string.as_str();
    console::log_2(&"got text".into(), &JsValue::from(text.len() as f32));

    console::log_1(&"loading mcap".into());

    // this just "unreachables" into the js console -- can we error-handle better?
    let mc = match magic_cap::Immutable::read(Cursor::new(text)) {
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
