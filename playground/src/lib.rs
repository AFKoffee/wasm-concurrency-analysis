use wasm_bindgen::prelude::*;
use wasm_ca_rs::{mutex::TracingMutex, thread::thread_spawn};


macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

//mod spawner;
//mod threadpool;

struct ThreadMetadata {
    id: String,
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
    let meta1 = ThreadMetadata {id: "1".into()};
    let meta2 = ThreadMetadata {id: "2".into()};

    thread_spawn(move || {
        deadlock_prone_task(&DATA_1, &DATA_2, &meta1)
    });

    thread_spawn(move || {
        deadlock_prone_task(&DATA_2, &DATA_1, &meta2)
    });
}

fn deadlock_prone_task(first: &TracingMutex<i64>, second: &TracingMutex<i64>, metadata: &ThreadMetadata) {
    console_log!("Worker {}: starting worker task ...", metadata.id);
    let mut i = 0;
    loop {
        increment_decrement(first, second);
        if i % 500 == 0 {
            console_log!("Worker {}: finished {i} iterations!", metadata.id)
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


