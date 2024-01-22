use crate::{ffi, Result};
use core::{ffi::c_void, ptr};

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
    pub fn buffer(&self) -> &[u8] {
        unsafe { ffi::from_raw_ffi_parts(self.m_buffer.cast(), self.m_size) }
    }

    #[must_use]
    pub fn buffer_mut(&mut self) -> &[u8] {
        unsafe { ffi::from_raw_ffi_parts_mut(self.m_buffer.cast(), self.m_size) }
    }

    /// Reallocate for a new size
    pub fn resize(&mut self, size: usize) -> Result<()> {
        unsafe { ffi::DirectXTexFFI_Blob_Resize(self.into(), size) }.success()
    }

    /// Shorten size without reallocation
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
        assert_eq!(blob.buffer().len(), 256);
        assert_eq!(blob.buffer_mut().len(), 256);

        blob.resize(128).unwrap();
        assert_eq!(blob.buffer().len(), 128);

        blob.trim(64).unwrap();
        assert_eq!(blob.buffer().len(), 64);

        assert!(blob.trim(128).is_err());
    }
}
