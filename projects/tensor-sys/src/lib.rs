use cudnn::{Cudnn, TensorDescriptor};
use cudnn::utils::{DataType, ScalParams};

#[test]
fn main() {
    // Initialize a new cuDNN context and allocates resources.
    let cudnn = Cudnn::new().unwrap();
    // Create a cuDNN Tensor Descriptor for `src` and `dest` memory.
    let src_desc = TensorDescriptor::new(&[2, 2, 2], &[4, 2, 1], DataType::Float).unwrap();
    let dest_desc = TensorDescriptor::new(&[2, 2, 2], &[4, 2, 1], DataType::Float).unwrap();
    let acti = cudnn.init_activation().unwrap();
    // Obtain the `src` and memory pointer on the GPU.
    // NOTE: You wouldn't do it like that. You need to really allocate memory on the GPU with e.g. CUDA or Collenchyma.
    let src_data: *const ::libc::c_void = ::std::ptr::null();
    let dest_data: *mut ::libc::c_void = ::std::ptr::null_mut();
    // Now you can compute the forward sigmoid activation on your GPU.
    cudnn.sigmoid_forward::<f32>(&acti, &src_desc, src_data, &dest_desc, dest_data, ScalParams::default()).unwrap();
    println!("{:?}", dest_data)
}