//! Module containing the [`ANSIStyledWriter`].

use crate::styling::{Change, Color, Color4Bit, Style, StyleChange, StyledWrite};
use std::{fmt, io};

/// A [`StyledWrite`] that only uses ANSI escape sequences.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ANSIStyledWriter<W: io::Write> {
    writer: W,
    style: Style,
}
impl<W: io::Write> ANSIStyledWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            style: Default::default(),
        }
    }
}
impl<W: io::Write> io::Write for ANSIStyledWriter<W> {
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

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.writer.write_fmt(fmt)
    }
}
impl<W: io::Write> StyledWrite for ANSIStyledWriter<W> {
    fn change_style(&mut self, change: StyleChange) -> io::Result<()> {
        if !change.any() {
            return Ok(());
        }

        let mut is_first = true;

        macro_rules! write_component {
            ($($arg:tt)*) => {
                {
                    if is_first {
                        write!(self.writer, "\x1B[")?;
                        #[allow(unused_assignments)]
                        {
                            is_first = false;
                        }
                    } else {
                        write!(self.writer, ";")?;
                    }
                    write!(self.writer, $($arg)*)?;
                }
            };
        }

        let (bold, dim) = if !matches!((&change.bold, &change.dim), (Change::Keep, Change::Keep)) {
            let bold = match change.bold {
                Change::Keep => self.style.bold,
                Change::SetTo(bold) => bold,
            };
            let dim = match change.dim {
                Change::Keep => self.style.dim,
                Change::SetTo(dim) => dim,
            };

            match (bold, dim) {
                (true, true) => write_component!("1;2"),
                (true, false) => write_component!("22;1"),
                (false, true) => write_component!("22;2"),
                (false, false) => write_component!("22"),
            }

            (bold, dim)
        } else {
            (self.style.bold, self.style.dim)
        };

        let italic = match change.italic {
            Change::Keep => self.style.italic,
            Change::SetTo(true) => {
                write_component!("3");
                true
            }
            Change::SetTo(false) => {
                write_component!("23");
                false
            }
        };

        let underline = match change.underline {
            Change::Keep => self.style.underline,
            Change::SetTo(true) => {
                write_component!("4");
                true
            }
            Change::SetTo(false) => {
                write_component!("24");
                false
            }
        };

        let blink = match change.blink {
            Change::Keep => self.style.blink,
            Change::SetTo(true) => {
                write_component!("5");
                true
            }
            Change::SetTo(false) => {
                write_component!("25");
                false
            }
        };

        let strike = match change.strike {
            Change::Keep => self.style.strike,
            Change::SetTo(true) => {
                write_component!("9");
                true
            }
            Change::SetTo(false) => {
                write_component!("29");
                false
            }
        };

        let foreground = match change.foreground {
            Change::Keep => self.style.foreground,
            Change::SetTo(foreground) => {
                match foreground {
                    Color::Unset => write_component!("39"),
                    Color::Color4Bit(color) => {
                        let color_number = color.intersection(Color4Bit::COLOR_MASK).bits();

                        if color.contains(Color4Bit::BRIGHT_BIT) {
                            write_component!("9{}", color_number);
                        } else {
                            write_component!("3{}", color_number);
                        }
                    }
                    Color::ANSI256(color) => write_component!("38;5;{}", color),
                    Color::RGB(r, g, b) => write_component!("38;2;{};{};{}", r, g, b),
                };
                foreground
            }
        };

        let background = match change.background {
            Change::Keep => self.style.background,
            Change::SetTo(background) => {
                match background {
                    Color::Unset => write_component!("49"),
                    Color::Color4Bit(color) => {
                        let color_number = color.intersection(Color4Bit::COLOR_MASK).bits();

                        if color.contains(Color4Bit::BRIGHT_BIT) {
                            write_component!("10{}", color_number);
                        } else {
                            write_component!("4{}", color_number);
                        }
                    }
                    Color::ANSI256(color) => write_component!("48;5;{}", color),
                    Color::RGB(r, g, b) => write_component!("48;2;{};{};{}", r, g, b),
                };
                background
            }
        };

        // At this point, it is certain that at least one component was written,
        // because this function returns early when `!changes.any()`.
        write!(self.writer, "m")?;

        self.style = Style {
            foreground,
            background,
            bold,
            dim,
            underline,
            italic,
            blink,
            strike,
        };

        Ok(())
    }

    fn reset_style(&mut self) -> io::Result<()> {
        write!(self.writer, "\x1B[0m")?;
        self.style = Default::default();
        Ok(())
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn swap_colors(&mut self) -> io::Result<()> {
        if self.style.foreground == self.style.background {
            return Ok(());
        }

        write!(self.writer, "\x1B[")?;

        match self.style.background {
            Color::Unset => write!(self.writer, "39")?,
            Color::Color4Bit(color) => {
                let color_number = color.intersection(Color4Bit::COLOR_MASK).bits();

                if color.contains(Color4Bit::BRIGHT_BIT) {
                    write!(self.writer, "9{}", color_number)?;
                } else {
                    write!(self.writer, "3{}", color_number)?;
                }
            }
            Color::ANSI256(color) => write!(self.writer, "38;5;{}", color)?,
            Color::RGB(r, g, b) => write!(self.writer, "38;2;{};{};{}", r, g, b)?,
        };

        write!(self.writer, ";")?;

        match self.style.foreground {
            Color::Unset => write!(self.writer, "49")?,
            Color::Color4Bit(color) => {
                let color_number = color.intersection(Color4Bit::COLOR_MASK).bits();

                if color.contains(Color4Bit::BRIGHT_BIT) {
                    write!(self.writer, "10{}", color_number)?;
                } else {
                    write!(self.writer, "4{}", color_number)?;
                }
            }
            Color::ANSI256(color) => write!(self.writer, "48;5;{}", color)?,
            Color::RGB(r, g, b) => write!(self.writer, "48;2;{};{};{}", r, g, b)?,
        };

        write!(self.writer, "m")?;

        std::mem::swap(&mut self.style.foreground, &mut self.style.background);

        Ok(())
    }
}
