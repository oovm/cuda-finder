use std::fs::read_dir;
use std::path::{Path, PathBuf};
use crate::{CudaFinder, CudaVersionInfo};

pub const CUDA_DEFAULT_WIN: &str = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\";
pub const CUDA_DEFAULT_LINUX: &str = "/usr/local/cuda/";


impl CudaFinder {
    pub fn search_cuda(&mut self) {
        self.cuda_in_default().ok();
        self.cuda_in_env();
    }
    fn cuda_in_env(&mut self) {
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

    pub fn cuda_in_default(&mut self) -> std::io::Result<()> {
        #[cfg(target_os = "windows")]
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

impl CudaVersionInfo {
    /// Find include path in `CUDA_HOME`
    pub fn include_path(&self) -> PathBuf {
        self.path.join("include").canonicalize().unwrap()
    }
    /// Find lib path in `CUDA_HOME`
    pub fn lib_path(&self) -> PathBuf {
        self.path.join("lib").canonicalize().unwrap()
    }
    /// Find bin path in `CUDA_HOME`
    pub fn bin_path(&self) -> PathBuf {
        self.path.join("bin").canonicalize().unwrap()
    }
}