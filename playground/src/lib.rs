use wasm_bindgen::prelude::*;
use web_sys::Worker;
use parking_lot::{Mutex, MutexGuard};

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

mod spawner;
//mod threadpool;

struct ThreadMetadata {
    id: String,
}

struct Work {
    func: Box<dyn FnOnce() -> Result<(), JsValue> /* + Send */>,
}

static DATA_1: SyncronizedData = SyncronizedData::new(0, 1);
static DATA_2: SyncronizedData = SyncronizedData::new(0,2);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);

    #[wasm_bindgen(js_namespace = utils)]
    fn register_new_thread() -> String;
}

#[wasm_bindgen]
pub async  fn create_deadlock() -> Result<(), JsValue> {
    console_log!("Main: starting attempt to deadlock.");
    
    let mut meta1 = ThreadMetadata {id: String::new()};
    let mut meta2 = ThreadMetadata {id: String::new()};

    let worker1 = spawner::spawn(&mut meta1)?;
    let worker2 = spawner::spawn(&mut  meta2)?;

    let work1 = Box::new(Work { func: Box::new(move || {
        deadlock_prone_task(&DATA_1, &DATA_2, &meta1)
    }) });

    let work2 = Box::new(Work { func: Box::new(move || {
        deadlock_prone_task(&DATA_2, &DATA_1, &meta2)
    }) });

    console_log!("Main: instanciated workloads for concurrency.");
    
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

fn deadlock_prone_task(first: &SyncronizedData, second: &SyncronizedData, metadata: &ThreadMetadata) -> Result<(), JsValue> {
    console_log!("Worker {}: starting worker task ...", metadata.id);
    let mut i = 0;
    loop {
        increment_decrement(first, second, metadata);
        if i % 500 == 0 {
            console_log!("Worker {}: {i} iterations done!", metadata.id)
        } 
        i += 1;
    }
    //console_log!("... finished!");
    //Ok(())
}

#[wasm_bindgen]
pub fn starting_attempt(thread_id: &str, mutex_idx: usize) {
    //Noop: Only to make locks visible to the outside
    console_log!("Thread {thread_id}: Attempting to lock Mutex {mutex_idx}!");
}


#[wasm_bindgen]
pub fn successful_attempt(thread_id: &str, mutex_idx: usize) {
    //Noop: Only to make locks visible to the outside
    console_log!("Thread {thread_id}: Aquired lock on Mutex {mutex_idx}!");
}

#[wasm_bindgen]
pub fn starting_release(thread_id: &str, mutex_idx: usize) {
    //Noop: Only to make locks visible to the outside
    console_log!("Thread {thread_id}: Releasing lock on Mutex {mutex_idx}!");
}


#[wasm_bindgen]
pub fn successfull_release(thread_id: &str, mutex_idx: usize) {
    //Noop: Only to make locks visible to the outside
    console_log!("Thread {thread_id}: Released lock on Mutex {mutex_idx}!");
}

struct SyncronizedData {
    data: Mutex<i64>,
    idx: usize,
}

impl SyncronizedData {
    pub const fn new(initial: i64, idx: usize) -> Self {
        Self { data: Mutex::new(initial), idx }
    }

    pub fn get_mut_data_ptr(&self, metadata: &ThreadMetadata, mutex_idx: usize) -> MutexGuard<'_, i64> {
        starting_attempt(&metadata.id, mutex_idx);
        let val = self.data.lock();
        successful_attempt(&metadata.id, mutex_idx);
        val
    }
}

impl Default for SyncronizedData {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

fn drop_guard(guard: MutexGuard<'_, i64>, metadata: &ThreadMetadata, mutex_idx: usize) {
    starting_release(&metadata.id, mutex_idx);
    drop(guard);
    successfull_release(&metadata.id, mutex_idx);
}

fn increment_decrement(first: &SyncronizedData, second: &SyncronizedData, metadata: &ThreadMetadata) {
    let mut first_data = first.get_mut_data_ptr(metadata, first.idx);
    let mut second_data = second.get_mut_data_ptr(metadata, second.idx);
    *first_data += 1;
    *second_data -= 1;
    drop_guard(first_data, metadata, first.idx);
    drop_guard(second_data, metadata, second.idx);
}


