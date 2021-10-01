//! Module containing the [`NotStyledWriter`].

use crate::styling::{Style, StyleChange, StyledWrite};
use std::io;

/// A [`StyledWrite`] that completely ignores style.
pub struct NotStyledWriter<W: io::Write> {
    pub writer: W,
    pub style: Style,
}
impl<W: io::Write> NotStyledWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            style: Default::default(),
        }
    }
}
impl<W: io::Write> io::Write for NotStyledWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.writer.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> io::Result<()> {
        self.writer.write_fmt(fmt)
    }
}
impl<W: io::Write> StyledWrite for NotStyledWriter<W> {
    fn change_style(&mut self, change: StyleChange) -> io::Result<()> {
        self.style = change.apply_to(&self.style);
        Ok(())
    }

    fn style(&self) -> &Style {
        &self.style
    }
}
