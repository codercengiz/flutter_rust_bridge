
use crate::frb_generated::RustAutoOpaque;
use serde::{Deserialize, Serialize};


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(Serialize, Deserialize)]
pub struct MyStruct {
    pub counter: i32,
    pub data: serde_json::Value,
}

impl MyStruct {
    
    pub fn new() -> Self {
        Self {
            counter: 0,
            data: serde_json::json!({}),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<MyStruct, serde_json::Error> {
        let device_message = serde_json::from_str(json)?;
        Ok(device_message)
    }
}
