use std::env::VarError;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use semver::Version;

use crate::CudaFinder;

mod cuda;
mod cudnn;


#[derive(Copy, Clone)]
pub struct CudaVersionInfo {
    version: Version,
    path: PathBuf,
}

pub struct CudnnVersionInfo {
    version: Version,
    path: PathBuf
}

pub struct CublasVersionInfo {}

pub struct CufftVersionInfo {}

pub struct CusolverVersionInfo {}

pub struct CusparseVersionInfo {}

pub struct NppVersionInfo {}
