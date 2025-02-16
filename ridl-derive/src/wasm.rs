use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_pojso(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = quote! {
        #[derive(ridl::Pojso, serde::Serialize, serde::Deserialize)]
        #[wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)]
        #input
    };
    TokenStream::from(output)
}

pub(crate) fn impl_pojso_macro_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("POJSO must be a struct with named fields"),
        },
        _ => panic!("POJSO must be a struct"),
    };

    #[cfg(not(feature = "json"))]
    let to_json_impl = quote! {};
    #[cfg(feature = "json")]
    let to_json_impl = quote! {
        #[wasm_bindgen]
        pub fn to_json(&self) -> Result<Vec<u8>, String> {
            serde_json::to_vec(self).map_err(|e| e.to_string())
        }
    };

    #[cfg(not(feature = "json"))]
    let from_json_impl = quote! {};
    #[cfg(feature = "json")]
    let from_json_impl = quote! {
        #[wasm_bindgen]
        pub fn from_json(json: &[u8]) -> Result<Self, String> {
            serde_json::from_slice(json).map_err(|e| e.to_string())
        }
    };

    #[cfg(not(feature = "msgpack"))]
    let to_msgpack_impl = quote! {};
    #[cfg(feature = "msgpack")]
    let to_msgpack_impl = quote! {
        #[wasm_bindgen]
        pub fn to_msgpack(&self) -> Result<Vec<u8>, String> {
            rmp_serde::to_vec(self).map_err(|e| e.to_string())
        }
    };

    #[cfg(not(feature = "msgpack"))]
    let from_msgpack_impl = quote! {};
    #[cfg(feature = "msgpack")]
    let from_msgpack_impl = quote! {
        #[wasm_bindgen]
        pub fn from_msgpack(msgpack: &[u8]) -> Result<Self, String> {
            rmp_serde::from_slice(msgpack).map_err(|e| e.to_string())
        }
    };

    let names = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let expanded = quote! {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl #name {
            #[wasm_bindgen(constructor)]
            pub fn new(#(#names: #types),*) -> Self {
                Self {
                    #(#names,)*
                }
            }

            #to_json_impl
            #from_json_impl
            #to_msgpack_impl
            #from_msgpack_impl
        }
    };

    TokenStream::from(expanded)
}
