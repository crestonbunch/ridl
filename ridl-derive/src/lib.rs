use proc_macro::TokenStream;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "py")]
mod py;

/// A procedural macro that applies #[derive(Popo)] and #[PyClass] to a struct.
#[cfg(feature = "py")]
#[proc_macro_attribute]
pub fn popo(attr: TokenStream, input: TokenStream) -> TokenStream {
    crate::py::impl_popo(attr, input)
}

/// A derive macro for easy implementation of POPOs.
#[cfg(feature = "py")]
#[proc_macro_derive(Popo)]
pub fn popo_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::py::impl_popo_macro_derive(&ast)
}

/// A procedural macro that applies #[derive(Pojso)] and #[wasm_bindgen] to a struct.
#[cfg(feature = "wasm")]
#[proc_macro_attribute]
pub fn pojso(attr: TokenStream, input: TokenStream) -> TokenStream {
    crate::wasm::impl_pojso(attr, input)
}

/// A derive macro for easy implementation of POJSOs.
#[cfg(feature = "wasm")]
#[proc_macro_derive(Pojso)]
pub fn pojso_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::wasm::impl_pojso_macro_derive(&ast)
}
