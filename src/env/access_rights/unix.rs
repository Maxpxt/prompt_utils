#![cfg(unix)]

/// Tells whether the current user is root.
pub fn is_root() -> bool {
    users::get_effective_uid() == 0
}
