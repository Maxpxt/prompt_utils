//! Utilities for querying, representing and manipulating information about the environment.

#[cfg(feature = "env-access_rights")]
pub mod access_rights;

#[cfg(feature = "env-command_result")]
pub mod command_result;

#[cfg(feature = "env-path")]
pub mod path;

#[cfg(feature = "env-python")]
pub mod python;

#[cfg(feature = "env-session")]
pub mod session;
