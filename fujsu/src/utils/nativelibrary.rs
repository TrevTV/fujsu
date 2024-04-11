// stolen from https://github.com/RinLovesYou/Ferrex
// thanks rin :)

use std::{
    ffi::{c_int, c_void, CStr},
    marker::PhantomData,
    ops::Deref,
    path::{Path, PathBuf},
};
use thiserror::Error;

/// possible library loading errors
#[derive(Debug, Error)]
pub enum LibError {
    /// failed to load library
    #[error("Failed to load library!")]
    FailedToLoadLib,

    /// failed to get lib name
    #[error("Failed to get lib name!")]
    FailedToGetLibName,

    /// failed to get lib path
    #[error("Failed to get lib path!")]
    FailedToGetLibPath,

    /// failed to get function pointer
    #[error("Failed to get function pointer: {0}")]
    FailedToGetFnPtr(String),

    #[error("Failed to create C-String")]
    FailedToCreateCString,
}

/// a representation of a permanently loaded library
#[derive(Debug, Clone)]
pub struct NativeLibrary {
    /// the name of the lib
    pub name: String,
    /// the path to the lib
    pub path: PathBuf,
    /// the pointer to the lib
    pub handle: *mut c_void,
}

impl NativeLibrary {
    /// gets a function pointer
    pub fn sym<T>(&self, name_str: &str) -> Result<NativeMethod<T>, LibError> {
        let display_string = name_str.to_string();

        let name = std::ffi::CString::new(name_str).map_err(|_| LibError::FailedToCreateCString)?;
        let ptr = unsafe { libc::dlsym(self.handle, name.as_ptr()) };
        if ptr.is_null() {
            return Err(LibError::FailedToGetFnPtr(display_string));
        }

        Ok(NativeMethod {
            inner: ptr.cast(),
            pd: PhantomData,
        })
    }

    pub fn sym_raw(&self, name_str: &str) -> Result<*mut c_void, LibError> {
        let display_string = name_str.to_string();

        let name = std::ffi::CString::new(name_str).map_err(|_| LibError::FailedToCreateCString)?;
        let ptr = unsafe { libc::dlsym(self.handle, name.as_ptr()) };
        if ptr.is_null() {
            return Err(LibError::FailedToGetFnPtr(display_string));
        }

        Ok(ptr)
    }
}

pub fn load_lib_with_dlerror<P: AsRef<Path>>(path: P, rtld: c_int) -> NativeLibrary {
    load_lib(path, rtld).unwrap_or_else(|e| {
        error!("Failed to load: {}", e.to_string());

        let dl_error = unsafe { libc::dlerror() };
        let error_message = unsafe { CStr::from_ptr(dl_error) };
        let formatted_string = error_message.to_string_lossy();
        error!("dlerror: {}", formatted_string);
        panic!();
    })
}

pub fn load_lib<P: AsRef<Path>>(path: P, rtld: c_int) -> Result<NativeLibrary, LibError> {
    use std::ffi::CString;

    let path = path.as_ref();

    let path_string = path.to_str().ok_or(LibError::FailedToGetLibPath)?;

    let c_path = CString::new(path_string).map_err(|_| LibError::FailedToCreateCString)?;

    let lib = unsafe { libc::dlopen(c_path.as_ptr(), rtld) };

    if lib.is_null() {
        return Err(LibError::FailedToLoadLib);
    }

    let lib_name = path
        .file_name()
        .ok_or(LibError::FailedToGetLibName)?
        .to_str()
        .ok_or(LibError::FailedToGetLibName)?
        .to_string();

    Ok(NativeLibrary {
        name: lib_name,
        path: path.to_path_buf(),
        handle: lib,
    })
}

pub fn load_self() -> Result<NativeLibrary, LibError> {
    let lib = unsafe { libc::dlopen(std::ptr::null(), libc::RTLD_NOW | libc::RTLD_GLOBAL) };

    if lib.is_null() {
        return Err(LibError::FailedToLoadLib);
    }

    Ok(NativeLibrary {
        name: "".into(),
        path: "".into(),
        handle: lib,
    })
}

#[derive(Debug)]
pub struct NativeMethod<T> {
    pub inner: *mut c_void,
    pd: PhantomData<T>,
}

unsafe impl<T: Send> Send for NativeMethod<T> {}
unsafe impl<T: Sync> Sync for NativeMethod<T> {}

impl<T> Clone for NativeMethod<T> {
    fn clone(&self) -> NativeMethod<T> {
        NativeMethod { ..*self }
    }
}

impl<T> Deref for NativeMethod<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(&self.inner as *const *mut _ as *const T) }
    }
}
