use super::*;

impl CudaVersionInfo {
    /// Find include path in `CUDNN_HOME`
    pub fn include_path(&self) -> PathBuf {
        self.path.join("include").canonicalize().unwrap()
    }
    /// Find lib path in `CUDNN_HOME`
    pub fn lib_path(&self) -> PathBuf {
        self.path.join("lib").canonicalize().unwrap()
    }
    /// Find bin path in `CUDNN_HOME`
    pub fn bin_path(&self) -> PathBuf {
        self.path.join("bin").canonicalize().unwrap()
    }
}