#[cfg(unix)]
#[path = "access_rights/unix.rs"]
mod unix;

#[cfg(windows)]
#[path = "access_rights/windows.rs"]
mod windows;

use std::io;

/// Tells whether the current process, on Unix, has a root user or, on Windows, is elevated.
pub fn is_root_or_elevated() -> io::Result<bool> {
    #[cfg(unix)]
    return Ok(unix::is_root());

    #[cfg(windows)]
    return windows::is_elevated();
}
