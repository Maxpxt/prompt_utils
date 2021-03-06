#[cfg(feature = "styling")]
#[macro_use]
pub mod styling;

#[cfg(any(
    feature = "writers",
    feature = "not_styled_writer",
    feature = "ansi_styled_writer"
))]
pub mod writers;

#[cfg(any(
    feature = "env",
    feature = "env-access_rights",
    feature = "env-command_result",
    feature = "env-git",
    feature = "env-path",
    feature = "env-python",
    feature = "env-session",
))]
pub mod env;

#[cfg(any(
    feature = "fmt",
    feature = "fmt-command_result",
    feature = "fmt-duration",
    feature = "fmt-git",
    feature = "fmt-path",
))]
pub mod fmt;
