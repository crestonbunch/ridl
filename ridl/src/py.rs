pub use ridl_impl::py::*;

#[macro_export]
macro_rules! impl_py {
    ($module_name:literal) => {
        #[pyo3::pymodule(name = $module_name)]
        pub fn root_py_module(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
            for module in inventory::iter::<ridl::py::RegisterPyModule> {
                module.0(m)?;
            }
            Ok(())
        }
    };
}
