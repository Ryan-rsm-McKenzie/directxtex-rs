use crate::{ffi, HResultError};
use core::{
    ffi::c_void,
    ptr::{self, NonNull},
    slice,
};

type Result<T> = core::result::Result<T, HResultError>;

#[derive(Debug)]
#[repr(C)]
pub struct Blob {
    m_buffer: *mut c_void,
    m_size: usize,
}

impl Blob {
    pub fn initialize(&mut self, size: usize) -> Result<()> {
        unsafe { ffi::DirectXTexFFI_Blob_Initialize(self.into(), size) }.success()
    }

    pub fn release(&mut self) {
        unsafe { ffi::DirectXTexFFI_Blob_Release(self.into()) }
    }

    fn do_get_buffer(&self) -> (NonNull<u8>, usize) {
        let len = unsafe { ffi::DirectXTexFFI_Blob_GetBufferSize(self.into()) };
        let ptr = unsafe { ffi::DirectXTexFFI_Blob_GetBufferPointer(self.into()) };
        let ptr = NonNull::new(ptr.cast::<u8>()).unwrap_or(NonNull::dangling());
        (ptr, len)
    }

    pub fn get_buffer(&self) -> &[u8] {
        let (ptr, len) = self.do_get_buffer();
        unsafe { slice::from_raw_parts(ptr.as_ptr(), len) }
    }

    pub fn get_buffer_mut(&mut self) -> &[u8] {
        let (ptr, len) = self.do_get_buffer();
        unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), len) }
    }

    pub fn resize(&mut self, size: usize) -> Result<()> {
        unsafe { ffi::DirectXTexFFI_Blob_Resize(self.into(), size) }.success()
    }

    pub fn trim(&mut self, size: usize) -> Result<()> {
        unsafe { ffi::DirectXTexFFI_Blob_Trim(self.into(), size) }.success()
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self {
            m_buffer: ptr::null_mut(),
            m_size: 0,
        }
    }
}

impl Drop for Blob {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi, Blob};
    use core::mem;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<Blob>(), unsafe {
            ffi::DirectXTexFFI_Blob_Sizeof()
        });
        assert_eq!(mem::align_of::<Blob>(), unsafe {
            ffi::DirectXTexFFI_Blob_Alignof()
        });
    }
}
