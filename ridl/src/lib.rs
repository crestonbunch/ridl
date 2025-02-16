#[cfg(feature = "py")]
pub mod py;

#[cfg(feature = "py")]
pub use ridl_derive::{popo, Popo};

#[cfg(feature = "wasm")]
pub use ridl_derive::{pojso, Pojso};
