#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications
)]

mod memory_store;
mod model;
mod sled_store;
mod store_utils;

use crate::memory_store::*;
use crate::model::*;
use crate::sled_store::*;
use pyo3::prelude::*;

/// Oxigraph library
#[pymodule]
fn oxigraph(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyNamedNode>()?;
    module.add_class::<PyBlankNode>()?;
    module.add_class::<PyLiteral>()?;
    module.add_class::<PyDefaultGraph>()?;
    module.add_class::<PyMemoryStore>()?;
    module.add_class::<PySledStore>()?;
    Ok(())
}