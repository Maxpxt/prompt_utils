#![cfg(windows)]

use std::{ffi::OsString, io, os::windows::ffi::OsStringExt, ptr};
use winapi::{
    shared::winerror::ERROR_INSUFFICIENT_BUFFER,
    um::{errhandlingapi::GetLastError, winbase::GetUserNameW},
};

const MAX_RETRIES: usize = 2;

pub fn query_username() -> io::Result<OsString> {
    unsafe {
        let mut len = 0;

        #[allow(unused_variables)]
        let result = GetUserNameW(ptr::null_mut(), &mut len);
        #[cfg(debug_assertions)]
        {
            assert_eq!(result, 0);
            assert_eq!(GetLastError(), ERROR_INSUFFICIENT_BUFFER);
        }

        let mut buf = Vec::with_capacity(len as usize);

        let mut retries = MAX_RETRIES;
        loop {
            let result = GetUserNameW(buf.as_mut_ptr(), &mut len);

            if result != 0 {
                buf.set_len(len as usize);
                break Ok(OsString::from_wide(&buf[0..buf.len() - 1]));
            } else {
                let error_code = GetLastError();

                if error_code != ERROR_INSUFFICIENT_BUFFER || retries == 0 {
                    break Err(io::Error::from_raw_os_error(error_code as _));
                } else {
                    buf.reserve(len as usize);
                    retries -= 1;
                }
            }
        }
    }
}
