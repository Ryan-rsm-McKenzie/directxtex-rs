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

    #[must_use]
    fn do_get_buffer(&self) -> (NonNull<u8>, usize) {
        let len = self.m_size;
        let ptr = NonNull::new(self.m_buffer.cast::<u8>()).unwrap_or(NonNull::dangling());
        (ptr, len)
    }

    #[must_use]
    pub fn get_buffer(&self) -> &[u8] {
        let (ptr, len) = self.do_get_buffer();
        unsafe { slice::from_raw_parts(ptr.as_ptr(), len) }
    }

    #[must_use]
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
        self.release();
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

    #[test]
    fn verify_api() {
        let mut blob = Blob::default();

        blob.initialize(256).unwrap();
        assert_eq!(blob.get_buffer().len(), 256);
        assert_eq!(blob.get_buffer_mut().len(), 256);

        blob.resize(128).unwrap();
        assert_eq!(blob.get_buffer().len(), 128);

        blob.trim(64).unwrap();
        assert_eq!(blob.get_buffer().len(), 64);

        assert!(blob.trim(128).is_err());
    }
}
