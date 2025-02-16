#[cfg(feature = "py")]
use pyo3::prelude::*;
#[cfg(feature = "py")]
use ridl::{popo, py::RegisterPyModule};

#[cfg(feature = "py")]
#[popo("hello")]
pub struct Hello {
    pub name: String,
}

#[cfg(feature = "py")]
#[test]
fn test_hello() {
    let mut hello = Hello::new("world".to_string());
    assert_eq!(hello.get_name().unwrap(), "world");
    hello.set_name("universe".to_string()).unwrap();
    assert_eq!(hello.get_name().unwrap(), "universe");
}

#[cfg(feature = "py")]
#[test]
fn test_inventory() {
    inventory::iter::<RegisterPyModule>
        .into_iter()
        .next()
        .unwrap();
}

#[cfg(feature = "py")]
#[cfg(feature = "json")]
#[test]
fn test_json() {
    let hello = Hello::new("world".to_string());
    let json = hello.to_json().unwrap();
    assert_eq!(json, r#"{"name":"world"}"#.as_bytes());
    let hello: Hello = Hello::from_json(json).unwrap();
    assert_eq!(hello.name, "world");
}

#[cfg(feature = "py")]
#[cfg(feature = "msgpack")]
#[test]
fn test_msgpack() {
    let hello = Hello::new("world".to_string());
    let msgpack = hello.to_msgpack().unwrap();
    let hello: Hello = Hello::from_msgpack(msgpack).unwrap();
    assert_eq!(hello.name, "world");
}
