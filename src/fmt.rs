//! Methods for formatting environment information.

#[cfg(feature = "fmt-command_result")]
pub mod command_result;

#[cfg(feature = "fmt-duration")]
pub mod duration;

#[cfg(feature = "fmt-git")]
pub mod git;

#[cfg(feature = "fmt-path")]
pub mod path;
