use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/script.js")]
extern {
    // Get request implemented by JavaScript code
    pub fn get(); 
}

#[wasm_bindgen]
pub struct Request {
    allow: bool,
}

#[wasm_bindgen]
impl Request {
    // Permissions are originally set to false
    pub fn new() -> Request {
        let allow = false;
        Request { 
            allow 
        }
    }
    // The user has the ability to provide permission
    pub fn allow(&mut self) {
        self.allow = !self.allow;
    }
    // Attempts to call parent get function
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
