use wasm_bindgen::prelude::*;
use wasm_ca_rs::{mutex::TracingMutex, thread::{self, thread_spawn}};


macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

static DATA_1: TracingMutex<i64> = TracingMutex::new(0);
static DATA_2: TracingMutex<i64> = TracingMutex::new(0);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

#[wasm_bindgen]
pub fn create_deadlock() {
    thread::set_bindgen_url_suffix(concat!("/pkg/", env!("CARGO_CRATE_NAME"), ".js").to_string());
    
    // Run a detached thread to to use join without freezing the main thread
    thread_spawn(|| {
        let _ = thread_spawn(||  console_log!("T{}: Testing Thread creation and termination", thread::thread_id())).join();

        let t1 = thread_spawn(move || {
            deadlock_prone_task(&DATA_1, &DATA_2)
        });

        let t2 = thread_spawn(move || {
            deadlock_prone_task(&DATA_2, &DATA_1)
        });

        console_log!("Thread {} waits for threads to finish ...", thread::thread_id());
        match t1.join() {
            Ok(()) => (),
            Err(e) => console_log!("Error in thread 1: {e:?}")
        };

        match t2.join() {
            Ok(()) => (),
            Err(e) => console_log!("Error in thread 2: {e:?}")
        };
    });
}

fn deadlock_prone_task(first: &TracingMutex<i64>, second: &TracingMutex<i64>) {
    console_log!("Worker {}: starting worker task ...", thread::thread_id());
    let mut i = 0;
    loop {
        increment_decrement(first, second);
        if i % 500 == 0 {
            console_log!("Worker {}: finished {i} iterations!", thread::thread_id())
        } 
        i += 1;
    }
}

fn increment_decrement(first: &TracingMutex<i64>, second: &TracingMutex<i64>) {
    let mut first_data = first.lock();
    let mut second_data = second.lock();
    *first_data += 1;
    *second_data -= 1;
    drop(first_data);
    drop(second_data);
}


