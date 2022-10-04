use std::env::VarError;
use std::fs::read_dir;
use std::path::Path;
use semver::Version;

use crate::CudaFinder;

mod cuda;
mod cudnn;

pub const CUDA_DEFAULT_WIN: &str = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\";
pub const CUDA_DEFAULT_LINUX: &str = "/usr/local/cuda/";

pub struct CudaVersionInfo {
    version: Version
}

pub struct CudnnVersionInfo {
    version: Version
}

pub struct CublasVersionInfo {

}

pub struct CufftVersionInfo {

}

pub struct CusolverVersionInfo {

}

pub struct CusparseVersionInfo {

}

pub struct NppVersionInfo {

}

impl CudaFinder {
    pub fn search_env(&mut self) {
        match std::env::var("CUDA_PATH") {
            Ok(_) => {}
            Err(_) => {}
        }
        match std::env::var("CUDA_PATH_V10_0") {
            Ok(_) => {}
            Err(_) => {}
        }
        match std::env::var("CUDA_PATH_V9_0") {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub fn search_preferred(&mut self) -> std::io::Result<()> {
        #[cfg(target_os = "cu_tools")]
        if Path::new(CUDA_DEFAULT_WIN).exists() {
            for dir in read_dir(CUDA_DEFAULT_WIN)? {
                let dir = dir?;
                println!("{:?}", dir.path());
            }
        }
        #[cfg(target_os = "linux")]
        if Path::new(CUDA_DEFAULT_LINUX).exists() {
            for dir in read_dir(CUDA_DEFAULT_LINUX)? {
                let dir = dir?;
                println!("{:?}", dir.path());
            }
        }
        Ok(())
    }
}


/// Load the CUDA path from the environment variable `CUDA_PATH`.
fn load_preferred_path() -> Option<String> {
    std::env::var("CUDA_PATH").ok()
}

// C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\

#[test]
fn test() {
    // find in default paths
    let mut cuda = CudaFinder::default();
    cuda.search_preferred().unwrap();
}