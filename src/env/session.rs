#[cfg(windows)]
#[path = "session/windows.rs"]
mod windows;

use std::{ffi::OsString, io};

/// Gets the name of the current user.
///
/// On Unix, it delegates to [`users::get_current_username`].
/// On Windows, it uses [GetUserNameW].
///
/// [GetUserNameW]: https://docs.microsoft.com/windows/win32/api/winbase/nf-winbase-getusernamew
pub fn query_username() -> io::Result<OsString> {
    #[cfg(unix)]
    return users::get_current_username().ok_or_else(|| io::ErrorKind::NotFound.into());

    #[cfg(windows)]
    return windows::query_username();
}

/// Gets the name of the host.
///
/// This simply delegates to [`hostname::get`].
pub fn query_hostname() -> io::Result<OsString> {
    hostname::get()
}
