#[cfg(feature = "styling")]
#[macro_use]
pub mod styling;

#[cfg(any(
    feature = "writers",
    feature = "not_styled_writer",
    feature = "ansi_styled_writer"
))]
pub mod writers;

#[cfg(any(feature = "env", feature = "env-path"))]
pub mod env;

#[cfg(any(feature = "fmt", feature = "fmt-path"))]
pub mod fmt;
