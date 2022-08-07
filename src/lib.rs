use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/script.js")]
extern {
    pub fn get();
}

#[wasm_bindgen]
pub struct Request {
    allow: bool,
}

#[wasm_bindgen]
impl Request {
    pub fn new() -> Request {
        let allow = false;
        Request { 
            allow 
        }
    }
    pub fn allow(&mut self) {
        self.allow = !self.allow;
    }
    pub fn try_get(&self) -> bool {
        if self.allow {
            get();
            return true;
        }
        else {
            return false;
        }
    }

}
