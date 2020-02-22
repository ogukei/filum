
use crate::vk::VkResult;

use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorCode {
    VkResult(VkResult),
    FFI(std::ffi::NulError),
    SuitablePhysicalDeviceNotFound,
    SuitableBufferMemoryTypeNotFound,
    ShaderLoadIO(std::io::Error),
    ShaderLoadUnaligned,
}

#[derive(Debug)]
pub struct Error {
    u: Box<ErrorCode>
}

impl From<VkResult> for Error {
    fn from(code: VkResult) -> Self {
        Error {
            u: Box::new(ErrorCode::VkResult(code))
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Error {
            u: Box::new(ErrorCode::FFI(error))
        }
    }
}

impl From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        Error {
            u: Box::new(code)
        }
    }
}

impl VkResult {
    pub fn into_result(self) -> Result<()> {
        if self == VkResult::VK_SUCCESS {
            Ok(())
        } else {
            Err(Error::from(self))
        }
    }
}