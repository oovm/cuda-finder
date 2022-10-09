use crate::CuToolsFinder;
use semver::Version;
use std::{
    env::VarError,
    fs::read_dir,
    path::{Path, PathBuf},
};

mod cuda;
mod cudnn;

#[derive(Clone, Debug)]
pub struct CudaVersionInfo {
    version: Version,
    path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CudnnVersionInfo {
    version: Version,
    path: PathBuf,
}

pub struct CublasVersionInfo {}

pub struct CufftVersionInfo {}

pub struct CusolverVersionInfo {}

pub struct CusparseVersionInfo {}

pub struct NppVersionInfo {}
