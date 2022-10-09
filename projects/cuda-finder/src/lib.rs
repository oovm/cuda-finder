pub use errors::{Error, Result};
use semver::Version;
use std::collections::BTreeMap;

mod cu_tools;
mod errors;

#[derive(Default)]
pub struct CuToolsFinder {
    cuda: BTreeMap<Version, CudaVersionInfo>,
    cudnn: BTreeMap<Version, CudnnVersionInfo>,
}

pub use crate::cu_tools::{
    CublasVersionInfo, CudaVersionInfo, CudnnVersionInfo, CufftVersionInfo, CusolverVersionInfo, CusparseVersionInfo,
    NppVersionInfo,
};
