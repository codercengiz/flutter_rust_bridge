use crate::api::simple;
use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;

pub struct MyParentStruct(pub RustAutoOpaque<simple::MyStruct>);

impl MyParentStruct {
    #[frb(sync)]
    pub fn from_json(json: &str) -> Result<MyParentStruct, serde_json::Error> {
        let device_message = simple::MyStruct::from_json(json)?;
        Ok(MyParentStruct(RustAutoOpaque::new(device_message)))
    }

    #[frb(sync)]
    pub fn to_json(&self) -> String {
        self.0.try_read().unwrap().to_json()
    }

    #[frb(sync)]
    pub fn new() -> Self {
        MyParentStruct(RustAutoOpaque::new(simple::MyStruct::new()))
    }

    #[frb(sync)]
    pub fn get_data(&self) -> serde_json::Value {
        self.0.try_read().unwrap().data.clone()
    }

    #[frb(sync)]
    pub fn compare_my_struct(&self, other: &MyParentStruct) -> bool {
        self.0.try_read().unwrap().to_json() == other.0.try_read().unwrap().to_json()
    }
}
