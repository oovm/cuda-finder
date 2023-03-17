pub use errors::{Error, Result};

mod errors;
mod windows;

#[derive(Default)]
pub struct CudaFinder {
    version: Option<String>,
}