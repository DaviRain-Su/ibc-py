use ibc::Signer as IbcSigner;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};

#[pyclass]
#[derive(FromPyObject)]
pub struct Signer(String);

impl From<IbcSigner> for Signer {
    fn from(value: IbcSigner) -> Self {
        Self(value.as_ref().to_string())
    }
}

#[pymethods]
impl Signer {
    #[new]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        // This is the equivalent of `self.__class__.__name__` in Python.
        let class_name: &str = slf.get_type().name()?;
        // To access fields of the Rust struct, we need to borrow the `PyCell`.
        Ok(format!("{}({})", class_name, slf.borrow().0))
    }

    // `__str__` is generally used to create an "informal" representation, so we
    // just forward to `i32`'s `ToString` trait implementation to print a bare number.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}
