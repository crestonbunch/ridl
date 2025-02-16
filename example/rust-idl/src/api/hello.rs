#[cfg_attr(feature = "py", ridl::popo("hello"))]
#[cfg_attr(feature = "wasm", ridl::pojso)]
#[derive(Debug)]
pub struct Hello {
    pub name: String,
}
