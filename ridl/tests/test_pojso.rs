#[cfg(feature = "wasm")]
use ridl::pojso;

#[cfg(feature = "wasm")]
#[pojso]
pub struct Hello {
    pub name: String,
}

#[cfg(feature = "wasm")]
#[test]
fn test_hello() {
    let hello = Hello::new("world".to_string());
    assert_eq!(hello.name, "world");
}

#[cfg(feature = "wasm")]
#[cfg(feature = "json")]
#[test]
fn test_json() {
    let hello = Hello::new("world".to_string());
    let json = hello.to_json().unwrap();
    assert_eq!(json, r#"{"name":"world"}"#.as_bytes());
    let hello: Hello = Hello::from_json(&json).unwrap();
    assert_eq!(hello.name, "world");
}

#[cfg(feature = "wasm")]
#[cfg(feature = "msgpack")]
#[test]
fn test_msgpack() {
    let hello = Hello::new("world".to_string());
    let msgpack = hello.to_msgpack().unwrap();
    let hello: Hello = Hello::from_msgpack(&msgpack).unwrap();
    assert_eq!(hello.name, "world");
}
