//! Some implementations of [`StyledWrite`][`crate::styling::StyledWrite`].

#[cfg(feature = "ansi_styled_writer")]
pub mod ansi;

#[cfg(feature = "not_styled_writer")]
pub mod not_styled;
