use std::{marker::PhantomData, ops::Deref, ptr::null_mut};

use libc::c_void;

use crate::errors::{hookerr::HookError, DynErr};

#[derive(Debug)]
pub struct NativeHook<T> {
    pub target: *mut c_void,
    pub trampoline: *mut c_void,
    pub detour: *mut c_void,
    pd: PhantomData<T>,
}

impl<T> NativeHook<T> {
    pub fn new(target: *mut c_void, detour: *mut c_void) -> Self {
        Self {
            target,
            trampoline: null_mut(),
            detour,
            pd: PhantomData,
        }
    }

    pub fn is_hooked(&self) -> bool {
        !self.target.is_null() && !self.trampoline.is_null()
    }

    pub fn hook(&mut self) -> Result<(), DynErr> {
        if self.is_hooked() {
            return Ok(());
        }

        let trampoline = hook(self.target as usize, self.detour as usize)?;

        self.trampoline = trampoline as *mut c_void;
        Ok(())
    }

    pub fn unhook(&self) -> Result<(), DynErr> {
        if !self.is_hooked() {
            return Ok(());
        }

        unhook(self.target as usize)?;

        Ok(())
    }
}

unsafe impl<T> Send for NativeHook<T> {}
unsafe impl<T> Sync for NativeHook<T> {}

impl<T> Clone for NativeHook<T> {
    fn clone(&self) -> Self {
        NativeHook { ..*self }
    }
}

impl<T> Deref for NativeHook<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(&self.trampoline as *const *mut _ as *const T) }
    }
}

pub fn hook(target: usize, detour: usize) -> Result<usize, HookError> {
    if target == 0 {
        return Err(HookError::Nullpointer("target".to_string()));
    }

    if detour == 0 {
        return Err(HookError::Nullpointer("detour".to_string()));
    }

    unsafe {
        let trampoline = dobby_rs::hook(target as dobby_rs::Address, detour as dobby_rs::Address)
            .map_err(|e| HookError::Failed(e.to_string()));

        let trampoline = match trampoline {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        if trampoline.is_null() {
            return Err(HookError::Null);
        }
        
        Ok(trampoline as usize)
    }
}

pub fn unhook(target: usize) -> Result<(), DynErr> {
    if target == 0 {
        return Err(HookError::Nullpointer("target".to_string()).into());
    }

    unsafe {
        dobby_rs::unhook(target as dobby_rs::Address)?;
    }

    Ok(())
}