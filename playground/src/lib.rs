use wasm_bindgen::prelude::*;
use web_sys::Worker;
use std::sync::{Mutex, MutexGuard};

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

mod spawner;
//mod threadpool;

struct Work {
    func: Box<dyn FnOnce() -> Result<(), JsValue> /* + Send */>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

#[wasm_bindgen]
pub async  fn create_deadlock() -> Result<(), JsValue> {
    let worker1 = spawner::spawn()?;
    let worker2 = spawner::spawn()?;

    let data1 = Box::new(SyncronizedData::default());
    let data2 = Box::new(SyncronizedData::default());

    let ptr1 = Box::into_raw(data1);
    let ptr2 = Box::into_raw(data2);

    let work1 = Box::new(Work { func: Box::new(move || {
        deadlock_prone_task(unsafe {&Box::from_raw(ptr1)}, unsafe {&Box::from_raw(ptr2)}, 1)
    }) });

    let work2 = Box::new(Work { func: Box::new(move || {
        deadlock_prone_task(unsafe {&Box::from_raw(ptr2)}, unsafe {&Box::from_raw(ptr1)}, 2)
    }) });

    console_log!("Instanciated work!");
    
    let res1 = execute_work(work1, &worker1);

    let res2 = execute_work(work2, &worker2);

    match res1.await {
        Ok(()) => (),
        Err(e) => return Err(e)
    };

    res2.await
}

async fn execute_work(work: Box<Work>, worker: &Worker) -> Result<(), JsValue> {
    let ptr = Box::into_raw(work);
    match worker.post_message(&JsValue::from(ptr as u32)) {
        Ok(()) => Ok(()),
        Err(e) => {
            unsafe { drop(Box::from_raw(ptr)); }
            crate::logv(&e);
            Err(e)
        }
    }
}

fn deadlock_prone_task(first: &SyncronizedData, second: &SyncronizedData, worker: u32) -> Result<(), JsValue> {
    console_log!("Worker {worker}: starting worker task ...");
    let mut i = 0;
    loop {
        increment_decrement(first, second);
        if i % 500 == 0 {
            console_log!("Worker {worker}: {i} iterations done!")
        } 
        i += 1;
    }
    //console_log!("... finished!");
    //Ok(())
}

struct SyncronizedData {
    data: Mutex<i64>
}

impl SyncronizedData {
    pub const fn new(initial: i64) -> Self {
        Self { data: Mutex::new(initial) }
    }

    pub fn get_mut_data_ptr(&self) -> MutexGuard<'_, i64> {
        self.data.lock().unwrap()
    }
}

impl Default for SyncronizedData {
    fn default() -> Self {
        Self::new(0)
    }
}

fn increment_decrement(first: &SyncronizedData, second: &SyncronizedData) {
    let mut first_data = first.get_mut_data_ptr();
    let mut second_data = second.get_mut_data_ptr();
    *first_data += 1;
    *second_data -= 1;
    drop(first_data);
    drop(second_data);
}


