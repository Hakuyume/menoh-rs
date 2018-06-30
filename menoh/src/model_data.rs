use menoh_sys;
use std::ffi;
use std::path;
use std::ptr;

use Error;
use error::check;

pub struct ModelData {
    handle: menoh_sys::menoh_model_data_handle,
}

impl ModelData {
    pub fn from_onnx<P>(path: P) -> Result<Self, Error>
        where P: AsRef<path::Path>
    {
        let path = ffi::CString::new(path.as_ref().as_os_str().to_string_lossy().as_ref())
            .map_err(|_| Error::NulError)?;
        let mut handle = ptr::null_mut();
        unsafe { check(menoh_sys::menoh_make_model_data_from_onnx(path.as_ptr(), &mut handle))? };
        Ok(Self { handle })
    }
}

impl Drop for ModelData {
    fn drop(&mut self) {
        unsafe { menoh_sys::menoh_delete_model_data(self.handle) }
    }
}
