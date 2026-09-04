use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader, Window, Location};
use magic_cap;
use std::io::Cursor;
use js_sys::{JsString, Object, Uint8Array};
use magic_cap::ReadCap;
use base64::prelude::*;

use reqwest;

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
async fn init() -> Result<(), JsValue> {
    // note: the name "init" above maps into the module, but is still
    // "the module default function" because we said
    // wasm_bindgen(start) above ..

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

    // ls **/*.rs|entr -r -s 'wasm-pack build --target web && python -m http.server'

    /*
    from reqwest examples

    let res = reqwest::Client::new()
        .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let text = res.text().await?;
     */

    let client = reqwest::Client::new();
    console::log_1(&"about to make request".into());
    //let result = client.get("http://localhost:8000/foo.txt").send().await?;
    let base = window.location().origin()?;

    let result = client.get(base + "/foo.txt").send().await?;
    console::log_1(&"made request".into());
    let val = document.create_element("div")?;
    //let code = result.text().await?;
    let code = result.text().await?;
    let text = format!("{:?}", code);
    val.set_inner_html(&text);
    body.append_child(&val)?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    //let url = format!("https://github.com/magic-cap/magic-cap/raw/refs/heads/main/kitten.mcap");
    let url = format!("kitten2.mcap");

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

    let mut done = false;
    while ! done {
        let mut result_value = JsFuture::from(reader.read()).await;
        if let Ok(stuff) = result_value {
            console::log_1(&"trying to load a chunk".into());
            let result: Object = stuff.dyn_into().unwrap();
            console::log_1(&format!("result is {:?}", result).into());
            let is_done = js_sys::Reflect::get(&result, &JsValue::from_str("done")).unwrap();
            if is_done.is_truthy() {
                done = true;
                continue;
            }
            let chunk_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
            // next line fails when we're done
            let chunk_array: Uint8Array = chunk_value.dyn_into().unwrap();
            let mut chunk = chunk_array.to_vec();
            console::log_1(&format!("got some {}", chunk.len()).into());
            data.append(&mut chunk);
        }
    }

    console::log_1(&"loading mcap".into());
    console::log_1(&format!("total mcap size: {}", data.len()).into());

    // this just "unreachables" into the js console -- can we error-handle better?
    let mut mc = magic_cap::Immutable::read(Cursor::new(data)).unwrap();

    console::log_1(&"loaded immutable".into());

    let blocks = format!("blocks: {:?}", mc.data_provider.total_blocks());
    console::log_1(&blocks.into());

    let maybe_readcap: Result<magic_cap::ImmutableReadCap, magic_cap::err::MagicCapError> = "mcap0r3LsgJf1LYZtRc_BGOzhx8j_FVDmFROmoBhDHGNTfXq8EAnU9NkykdwXfOg6VdQ7v".try_into();
    if let Ok(readcap) = maybe_readcap {
        console::log_1(&"made a readcap".into());
        let decrypted = readcap.decrypt(&mut mc);
        if let Ok(plain) = decrypted {
            // plain is a Vec<u8> and/or a slice if we want .. how do we give this to the DOM?

            let image = document.get_element_by_id("kitten")
                .unwrap()
                .dyn_into::<web_sys::HtmlImageElement>()
                .unwrap();
            console::log_1(&"decrypted".into());

            // try 0: can we make a data:* URL out of the bytes?
            // need: data:image/jpeg:base64
            let b64 = BASE64_STANDARD.encode(plain.as_slice());
            let dataurl = format!("data:image/jpeg;base64,{}", b64);
            image.set_src(dataurl.as_str());
            console::log_1(&format!("data url: {}", dataurl).into());
        }
    }

/*
    let foo = format!("mcap: {:?}", mc.metadata);
    let val = document.create_element("p")?;
    val.set_inner_html(foo.as_str());
    body.append_child(&val)?;
     */

    Ok(())
}

/*
#[wasm_bindgen]
struct CatalogApi {
    catalog: magic_cap::catalog::ImmutableWebCatalog,
}
*/

// what can we pass across the Divide to JS?
// ...what we _want_ to pass back is some "context" object
// and some methods

use wasm_bindgen::prelude::JsValue;

/*
#[wasm_bindgen]
pub fn create_catalog(url: Url) -> Result<JsValue, JsValue> {
    let rtn: JsValue = "just a string".into();
    Ok(rtn)
}
*/

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
