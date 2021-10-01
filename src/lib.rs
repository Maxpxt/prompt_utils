#[cfg(feature = "styling")]
#[macro_use]
pub mod styling;

#[cfg(any(
    feature = "writers",
    feature = "not_styled_writer",
    feature = "ansi_styled_writer"
))]
pub mod writers;

#[cfg(any(feature = "env", feature = "env-command_result", feature = "env-path"))]
pub mod env;

#[cfg(any(feature = "fmt", feature = "fmt-command_result", feature = "fmt-path"))]
pub mod fmt;
