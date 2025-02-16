use pyo3::prelude::*;

pub struct RegisterPyModule(pub fn(m: &Bound<'_, PyModule>) -> PyResult<()>);

impl RegisterPyModule {
    pub const fn new(f: fn(m: &Bound<'_, PyModule>) -> PyResult<()>) -> Self {
        Self(f)
    }
}

inventory::collect!(RegisterPyModule);
