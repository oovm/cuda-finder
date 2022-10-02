use std::fs::read_dir;
use std::path::Path;

use crate::CudaFinder;

pub const CUDA_DEFAULT_PATHS: &str = "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\";

impl CudaFinder {
    pub fn search_env(&mut self) {}

    pub fn search_preferred(&mut self) -> std::io::Result<()> {
        for dir in read_dir(CUDA_DEFAULT_PATHS)? {
            let dir = dir?;
            println!("{:?}", dir.path());
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