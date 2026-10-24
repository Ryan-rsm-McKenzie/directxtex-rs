use crate::{HResultError, Result};

#[cfg(windows)]
use core::ffi::c_void;

#[cfg(windows)]
use crate::ffi::MutNonNull;

#[cfg(windows)]
use windows::core::Interface;

#[cfg(windows)]
use windows::Win32::Foundation::HMODULE;

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Device,
    D3D11_CREATE_DEVICE_FLAG,
    D3D11_CREATE_DEVICE_DEBUG,
    D3D11_SDK_VERSION,
};

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::{
    D3D_DRIVER_TYPE_HARDWARE,
    D3D_FEATURE_LEVEL,
    D3D_FEATURE_LEVEL_10_0,
    D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_11_0,
};

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::D3D11CreateDevice;

pub struct D3D11Device {
    #[cfg(windows)]
    device: Option<ID3D11Device>,
    #[cfg(not(windows))]
    _pd: core::marker::PhantomData<()>,
}

impl D3D11Device {
    pub const fn is_available() -> bool {
        cfg!(windows)
    }

    #[allow(clippy::unused_self)]
    pub const fn is_initialized(&self) -> bool {
        #[cfg(windows)]
        {
            self.device.is_some()
        }

        #[cfg(not(windows))]
        false
    }

    #[allow(clippy::unused_self)]
    pub fn is_healthy(&self) -> bool {
        #[cfg(windows)]
        unsafe {
            self.device.as_ref().is_some_and(|dev| dev.GetDeviceRemovedReason().is_ok())
        }

        #[cfg(not(windows))]
        false
    }

    #[cfg(windows)]
    pub(crate) fn as_ffi_ptr(&mut self) -> Option<MutNonNull<c_void>> {
        return self.device.as_mut().map(|dev| {
            unsafe{
                &mut *dev.as_raw()
            }.into()
        });
    }

    #[cfg(windows)]
    pub fn new() -> Result<Self> {
        let mut device = core::mem::MaybeUninit::zeroed();

        let feature_levels: [D3D_FEATURE_LEVEL; 3] = [
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
        ];

        let mut flags = D3D11_CREATE_DEVICE_FLAG::default();
        #[cfg(debug_assertions)]
        {
            flags |= D3D11_CREATE_DEVICE_DEBUG;
        }

        let device = unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                flags,
                Some(&feature_levels),
                D3D11_SDK_VERSION,
                Some(device.as_mut_ptr()),
                None,
                None,
            ).map_err(|e| HResultError::from(e.code()))?;
            device.assume_init()
        };

        Ok(Self { device })
    }

    #[cfg(not(windows))]
    pub fn new() -> Result<Self> {
        Err(Self::not_available_error())
    }

    #[cfg(not(windows))]
    pub(crate) fn not_available_error() -> HResultError {
        crate::hresult::HResult::from(0x80004002).success(0).err().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::d3d11::{D3D11Device};

    #[test]
    fn verify_d3d11_device() {
        if cfg!(windows) {
            assert!(D3D11Device::is_available());
            let d3d11_device = D3D11Device::new().unwrap();
            assert!(d3d11_device.is_initialized());
            assert!(d3d11_device.is_healthy());
        } else{
            assert!(!D3D11Device::is_available());
            assert!(D3D11Device::new().is_err());
        }

    }
}
