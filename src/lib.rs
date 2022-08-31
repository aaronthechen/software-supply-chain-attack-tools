use wasm_bindgen::prelude::*;

// Import from parent JavaScript module
#[wasm_bindgen(module = "/script.js")]
extern {
    // Get request implemented by JavaScript code
    pub fn get(); 
}

// Export to WebAssembly
#[wasm_bindgen]
pub struct Request {
    allow: bool,
}

//Export to WebAssembly
#[wasm_bindgen]
impl Request {
    pub fn new() -> Request {
        // Permissions are originally set to false
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
