use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, Worker};

use crate::{register_new_thread, ThreadMetadata, Work};

pub fn spawn(context: &mut ThreadMetadata) -> Result<Worker, JsValue> {
    let worker = Worker::new("./worker.js")?;
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array)?;
    context.id = register_new_thread();
    console_log!("Main: created a new worker from within wasm. Thread ID: {}", context.id);
    Ok(worker)
}

#[wasm_bindgen]
pub fn worker_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)()?;
    //global.post_message(&JsValue::undefined())?;
    console_log!("Worker <?>: terminating ...");
    global.close();
    Ok(())
}