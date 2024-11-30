use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, ErrorEvent, Event, MessageEvent, Worker};

// Inspired by: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/raytrace-parallel/src/pool.rs

struct Work {
    func: Box<dyn FnOnce() + Send>,
}

#[wasm_bindgen]
pub struct WorkerPool {
    state: Rc<PoolState>
}

struct PoolState {
    workers: RefCell<Vec<Worker>>,
    callback: Closure<dyn FnMut(Event)>
}

#[wasm_bindgen]
impl WorkerPool {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: usize) -> Result<Self, JsValue> {
        let pool = Self {
            state: Rc::new(PoolState {
                workers: RefCell::new(Vec::with_capacity(initial)),
                callback: Closure::new(|event: Event| {
                    console_log!("unhandled event: {}", event.type_());
                    crate::logv(&event);
                }),
            }),
        };
        for _ in 0..initial {
            let worker = pool.spawn()?;
            pool.state.push(worker);
        }

        Ok(pool)
    }

    fn spawn(&self) -> Result<Worker, JsValue> {
        console_log!("spawning new worker");

        let worker = Worker::new("./poolworker.js")?;

        let array = js_sys::Array::new();
        array.push(&wasm_bindgen::module());
        array.push(&wasm_bindgen::memory());
        worker.post_message(&array)?;
        Ok(worker)
    }

    fn worker(&self) -> Result<Worker, JsValue> {
        match self.state.workers.borrow_mut().pop() {
            Some(worker) => Ok(worker),
            None => self.spawn(),
        }
    }

    fn execute(&self, f: impl FnOnce() + Send + 'static) -> Result<Worker, JsValue> {
        let worker = self.worker()?;
        let work = Box::new(Work { func: Box::new(f) });
        let ptr = Box::into_raw(work);
        match worker.post_message(&JsValue::from(ptr as u32)) {
            Ok(()) => Ok(worker),
            Err(e) => {
                unsafe {
                    drop(Box::from_raw(ptr));
                }
                Err(e)
            }
        }
    }

    fn reclaim_on_message(&self, worker: Worker) {
        let state = Rc::downgrade(&self.state);
        let worker2 = worker.clone();
        let reclaim_slot = Rc::new(RefCell::new(None));
        let slot2 = reclaim_slot.clone();
        let reclaim = Closure::<dyn FnMut(_)>::new(move |event: Event| {
            if let Some(error) = event.dyn_ref::<ErrorEvent>() {
                console_log!("error in worker: {}", error.message());
                // TODO: this probably leaks memory somehow? It's sort of
                // unclear what to do about errors in workers right now.
                return;
            }

            // If this is a completion event then can deallocate our own
            // callback by clearing out `slot2` which contains our own closure.
            if let Some(_msg) = event.dyn_ref::<MessageEvent>() {
                if let Some(state) = state.upgrade() {
                    state.push(worker2.clone());
                }
                *slot2.borrow_mut() = None;
                return;
            }

            console_log!("unhandled event: {}", event.type_());
            crate::logv(&event);
            // TODO: like above, maybe a memory leak here?
        });
        worker.set_onmessage(Some(reclaim.as_ref().unchecked_ref()));
        *reclaim_slot.borrow_mut() = Some(reclaim);
    }
}

// NOTE: no wasm-bindgen declaration here
impl WorkerPool {
    pub fn run(&self, f: impl FnOnce() + Send + 'static) -> Result<(), JsValue> {
        let worker = self.execute(f)?;
        self.reclaim_on_message(worker);
        Ok(())
    }
}


impl PoolState {
    fn push(&self, worker: Worker) {
        worker.set_onmessage(Some(self.callback.as_ref().unchecked_ref()));
        worker.set_onerror(Some(self.callback.as_ref().unchecked_ref()));
        let mut workers = self.workers.borrow_mut();
        for prev in workers.iter() {
            let prev: &JsValue = prev;
            let worker: &JsValue = &worker;
            assert!(prev != worker);
        }
        workers.push(worker);
    }
}

#[wasm_bindgen]
pub fn child_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)();
    global.post_message(&JsValue::undefined())?;
    Ok(())
}
