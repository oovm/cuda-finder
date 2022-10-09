use super::*;

impl CuToolsFinder {
    pub fn find_cudnn(&mut self) {}

    fn cudnn_in_env(&mut self) {
        match std::env::var("CUDNN_HOME") {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    fn cudnn_in_cuda(&mut self) {
        for cuda in self.cuda.values() {
            match search_in_cuda(cuda) {
                Ok(o) => {
                    self.cudnn.insert(o.version.clone(), o);
                }
                Err(_) => {}
            }
        }
    }
}

fn search_in_cuda(info: &CudaVersionInfo) -> Result<CudnnVersionInfo, VarError> {
    let root = info.path.as_path();
    todo!("{:?}", root)
}

impl CudnnVersionInfo {
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
