use std::sync::Mutex;
use std::sync::PoisonError;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

pub struct CurrentStatus {
    pub index: i32,
}

impl CurrentStatus {
    fn new() -> Self {
        CurrentStatus { index: 1 }
    }
    fn get_index(&mut self) -> i32 {
        self.index += 1;
        self.index
    }

    fn add_index(&mut self) {
        self.index += 2;
    }
}

lazy_static! {
    pub static ref FOO: Mutex<CurrentStatus> = Mutex::new(CurrentStatus::new());
}

unsafe impl Send for CurrentStatus {}

#[wasm_bindgen]
pub fn add_index() {
    FOO.lock()
        .unwrap_or_else(PoisonError::into_inner)
        .add_index();
}

#[wasm_bindgen]
pub fn get_index() -> i32 {
    let mut foo = FOO.lock().unwrap_or_else(PoisonError::into_inner);
    foo.get_index()
}