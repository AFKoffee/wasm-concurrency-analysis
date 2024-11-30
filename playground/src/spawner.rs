use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, Worker};

use crate::Work;

pub fn spawn() -> Result<Worker, JsValue> {
    let worker = Worker::new("./worker.js")?;
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array)?;
    console_log!("Created a new worker from within Wasm");
    Ok(worker)
}

#[wasm_bindgen]
pub fn worker_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)()?;
    //global.post_message(&JsValue::undefined())?;
    global.close();
    Ok(())
}