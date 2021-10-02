#![cfg(windows)]

use std::{
    io,
    mem::{self, MaybeUninit},
    ptr,
};

use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{GetCurrentProcess, OpenProcessToken},
    securitybaseapi::GetTokenInformation,
    winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
};

/// Tells whether the current process is elevated.
pub fn is_elevated() -> io::Result<bool> {
    // Adapted from https://vimalshekar.github.io/codesamples/Checking-If-Admin
    // accessed on 2021-10-01

    unsafe {
        let mut token_handle = ptr::null_mut();

        let result = {
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
                Err(io::Error::last_os_error())
            } else {
                let mut elevation = MaybeUninit::<TOKEN_ELEVATION>::uninit();
                let mut size = MaybeUninit::uninit();

                if GetTokenInformation(
                    token_handle,
                    TokenElevation,
                    elevation.as_mut_ptr() as *mut _,
                    mem::size_of::<TOKEN_ELEVATION>() as _,
                    size.as_mut_ptr(),
                ) == 0
                {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(elevation.assume_init().TokenIsElevated != 0)
                }
            }
        };

        if !token_handle.is_null() {
            CloseHandle(token_handle);
        }

        result
    }
}
