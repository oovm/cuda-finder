use std::collections::BTreeMap;
use semver::Version;
pub use errors::{Error, Result};

mod errors;
mod cu_tools;

#[derive(Default)]
pub struct CudaFinder {
    cuda: BTreeMap<Version, CudaVersionInfo>,
}

pub use crate::cu_tools::{CudaVersionInfo, CudnnVersionInfo, CublasVersionInfo, CufftVersionInfo, CusolverVersionInfo, CusparseVersionInfo, NppVersionInfo};