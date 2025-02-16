use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

pub(crate) fn impl_popo(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: &syn::DeriveInput = &syn::parse(input.clone()).unwrap();
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(_) => (),
        _ => panic!("POPO must be a struct"),
    };

    let fn_name = format_ident!("pymodule_register_{}", name.to_string().to_lowercase());
    let input = proc_macro2::TokenStream::from(input);
    let python_module = parse_macro_input!(attr as syn::LitStr).value();
    let output = quote! {
        #[derive(ridl::Popo, serde::Serialize, serde::Deserialize)]
        #[pyo3::pyclass]
        #input

        pub fn #fn_name(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
            use pyo3::prelude::PyModuleMethods;
            let py = m.py();
            let module = pyo3::types::PyModule::new(py, #python_module)?;
            module.add_class::<Hello>()?;
            m.add_submodule(&module)?;
            Ok(())
        }

        inventory::submit!(ridl::py::RegisterPyModule::new(#fn_name));
    };
    TokenStream::from(output)
}

pub(crate) fn impl_popo_macro_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("POPO must be a struct with named fields"),
        },
        _ => panic!("POPO must be a struct"),
    };

    let names = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let setter_names = names
        .iter()
        .map(|n| format_ident!("set_{}", n))
        .collect::<Vec<_>>();
    let getter_names = names
        .iter()
        .map(|n| format_ident!("get_{}", n))
        .collect::<Vec<_>>();
    let types = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    #[cfg(not(feature = "json"))]
    let to_json_impl = quote! {};
    #[cfg(feature = "json")]
    let to_json_impl = quote! {
        pub fn to_json(&self) -> pyo3::PyResult<Vec<u8>> {
            serde_json::to_vec(self).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
        }
    };

    #[cfg(not(feature = "json"))]
    let from_json_impl = quote! {};
    #[cfg(feature = "json")]
    let from_json_impl = quote! {
        #[staticmethod]
        pub fn from_json(data: Vec<u8>) -> pyo3::PyResult<Self> {
            serde_json::from_slice(&data).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
        }
    };

    #[cfg(not(feature = "msgpack"))]
    let to_msgpack_impl = quote! {};
    #[cfg(feature = "msgpack")]
    let to_msgpack_impl = quote! {
        pub fn to_msgpack(&self) -> pyo3::PyResult<Vec<u8>> {
            rmp_serde::to_vec(self).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
        }
    };

    #[cfg(not(feature = "msgpack"))]
    let from_msgpack_impl = quote! {};
    #[cfg(feature = "msgpack")]
    let from_msgpack_impl = quote! {
        #[staticmethod]
        pub fn from_msgpack(data: Vec<u8>) -> pyo3::PyResult<Self> {
            rmp_serde::from_slice(&data).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
        }
    };

    let expanded = quote! {
        #[pyo3::pymethods]
        impl #name {
            #[new]
            pub fn new(#(#names: #types),*) -> Self {
                Self {
                    #(#names,)*
                }
            }

            #(#[getter]
            pub fn #getter_names(&self) -> pyo3::PyResult<&str> {
                Ok(&self.#names)
            }

            #[setter]
            pub fn #setter_names(&mut self, #names: #types) -> pyo3::PyResult<()> {
                self.#names = #names;
                Ok(())
            })*

            #to_json_impl
            #from_json_impl
            #to_msgpack_impl
            #from_msgpack_impl
        }
    };

    TokenStream::from(expanded)
}
