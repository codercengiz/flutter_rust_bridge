use crate::api::simple;
use crate::frb_generated::{MoiArcPool, MoiArcValue, RustAutoOpaque};
use flutter_rust_bridge::for_generated::RustAutoOpaqueInner;
use flutter_rust_bridge::frb;

struct MyStruct (RustAutoOpaque<simple::MyStruct>);
flutter_rust_bridge::frb_generated_moi_arc_impl_value!(
    RustAutoOpaqueInner<simple::MyStruct>
);

impl MyStruct {
    #[ frb(sync)]
    pub fn from_json(json: &str) -> Result<MyStruct, serde_json::Error> {
        let device_message = simple::MyStruct::from_json(json)?;
        Ok(MyStruct(RustAutoOpaque::new(device_message)))
    }

    #[ frb(sync)]
    pub fn to_json(&self) -> String {
        self.0.try_read().unwrap().to_json()
    }

    #[ frb(sync)]
    pub fn new() -> Self {
        MyStruct(RustAutoOpaque::new(simple::MyStruct::new(

        )))
    }
}
