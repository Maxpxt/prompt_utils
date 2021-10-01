//! Formatting of [`Path`]s.

use crate::styling::StyledWrite;
use std::{
    fmt, io,
    path::{Component, Path},
};

/// Writes a path in its full form.
///
/// `separator` is the path separator.
///
/// `root_separator` is the path separator used following the root dir.
/// If the root dir is displayed the same as `separator`,
/// setting `root_separator` to an empty string is likely desired,
/// otherwise, for example, the path `/Users/Shared` would display as `//Users/Shared`
/// (assuming the root dir displays as `/` and `separator` is `/`).
///
/// `root_dir_override` will, if provided, be displayed instead of the root dir.
/// This may be useful in Windows, where the root is shown as `\` by default.
pub fn write_full(
    writer: &mut (impl StyledWrite + ?Sized),
    path: &Path,
    separator: impl fmt::Display,
    root_separator: impl fmt::Display,
    root_dir_override: Option<impl fmt::Display + Copy>,
) -> io::Result<()> {
    let mut components = path.components();

    loop {
        match components.next() {
            Some(Component::Prefix(prefix)) => {
                write!(writer, "{}", prefix.as_os_str().to_string_lossy())?;
            }
            Some(Component::RootDir) => {
                if let Some(root_dir_override) = root_dir_override {
                    write!(writer, "{}", root_dir_override)?;
                } else {
                    write!(
                        writer,
                        "{}",
                        Component::RootDir.as_os_str().to_string_lossy(),
                    )?;
                }
                // No need to check for `Prefix` or `RootDir` here
                // because `RootDir` is guaranteed to
                // appear after any prefix and before anything else
                // (see https://doc.rust-lang.org/std/path/enum.Component.html#variant.RootDir)
                match components.next() {
                    Some(component) => {
                        write!(
                            writer,
                            "{}{}",
                            root_separator,
                            component.as_os_str().to_string_lossy(),
                        )?;

                        for component in components {
                            write!(
                                writer,
                                "{}{}",
                                separator,
                                component.as_os_str().to_string_lossy(),
                            )?;
                        }

                        break Ok(());
                    }
                    None => break Ok(()),
                }
            }
            Some(component) => {
                write!(writer, "{}", component.as_os_str().to_string_lossy())?;

                // No need to check for `Prefix` or `RootDir` here
                // because `RootDir` is guaranteed to
                // appear after any prefix and before anything else
                // (see https://doc.rust-lang.org/std/path/enum.Component.html#variant.RootDir)
                for component in components {
                    write!(
                        writer,
                        "{}{}",
                        separator,
                        component.as_os_str().to_string_lossy(),
                    )?;
                }

                break Ok(());
            }
            None => break Ok(()),
        }
    }
}

/// Writes a path with all intermediate folders replaced by `replacement`.
///
/// `separator` is the path separator.
///
/// `root_separator` is the path separator used following the root dir.
/// If the root dir is displayed the same as `separator`,
/// setting `root_separator` to an empty string is likely desired,
/// otherwise, for example, the path `/Users/Shared` would display as `//Users/Shared`
/// (assuming the root dir displays as `/` and `separator` is `/`).
///
/// `root_dir_override` will, if provided, be displayed instead of the root dir.
/// This may be useful in Windows, where the root is shown as `\` by default.
///
/// `replacement` is the replacement for intermediate folders.
pub fn write_with_middle_hidden(
    writer: &mut (impl StyledWrite + ?Sized),
    path: &Path,
    separator: impl fmt::Display,
    root_separator: impl fmt::Display,
    root_dir_override: Option<impl fmt::Display + Copy>,
    replacement: impl fmt::Display,
) -> io::Result<()> {
    let mut components = path.components();
    loop {
        match components.next() {
            Some(Component::Prefix(prefix)) => {
                write!(writer, "{}", prefix.as_os_str().to_string_lossy())?;
            }
            Some(first) => match components.next_back() {
                Some(last) => {
                    match first {
                        Component::RootDir => {
                            if let Some(root_dir_override) = root_dir_override {
                                write!(writer, "{}", root_dir_override)?;
                            } else {
                                write!(
                                    writer,
                                    "{}",
                                    Component::RootDir.as_os_str().to_string_lossy(),
                                )?;
                            }
                            write!(writer, "{}", root_separator)?;
                        }
                        _ => write!(
                            writer,
                            "{}{}",
                            first.as_os_str().to_string_lossy(),
                            separator,
                        )?,
                    }

                    for _ in 0..components.count() {
                        write!(writer, "{}{}", replacement, separator)?;
                    }

                    write!(writer, "{}", last.as_os_str().to_string_lossy())?;

                    break Ok(());
                }
                None => break write!(writer, "{}", first.as_os_str().to_string_lossy()),
            },
            None => break Ok(()),
        }
    }
}

/// Writes a path with all intermediate folders replaced by a single instance of `replacement`.
///
/// `separator` is the path separator.
///
/// `root_separator` is the path separator used following the root dir.
/// If the root dir is displayed the same as `separator`,
/// setting `root_separator` to an empty string is likely desired,
/// otherwise, for example, the path `/Users/Shared` would display as `//Users/Shared`
/// (assuming the root dir displays as `/` and `separator` is `/`).
///
/// `root_dir_override` will, if provided, be displayed instead of the root dir.
/// This may be useful in Windows, where the root is shown as `\` by default.
///
/// `replacement` is the replacement for all intermediate folders.
pub fn write_short(
    writer: &mut (impl StyledWrite + ?Sized),
    path: &Path,
    separator: impl fmt::Display,
    root_separator: impl fmt::Display,
    root_dir_override: Option<impl fmt::Display + Copy>,
    replacement: impl fmt::Display,
) -> io::Result<()> {
    let mut components = path.components();
    loop {
        match components.next() {
            Some(Component::Prefix(prefix)) => {
                write!(writer, "{}", prefix.as_os_str().to_string_lossy())?;
            }
            Some(first) => match components.next_back() {
                Some(last) => {
                    match first {
                        Component::RootDir => {
                            if let Some(root_dir_override) = root_dir_override {
                                write!(writer, "{}", root_dir_override)?;
                            } else {
                                write!(
                                    writer,
                                    "{}",
                                    Component::RootDir.as_os_str().to_string_lossy(),
                                )?;
                            }
                            write!(writer, "{}", root_separator)?;
                        }
                        _ => write!(
                            writer,
                            "{}{}",
                            first.as_os_str().to_string_lossy(),
                            separator,
                        )?,
                    }

                    if components.next().is_some() {
                        write!(writer, "{}{}", replacement, separator)?;
                    }

                    write!(writer, "{}", last.as_os_str().to_string_lossy())?;

                    break Ok(());
                }
                None => break write!(writer, "{}", first.as_os_str().to_string_lossy()),
            },
            None => break Ok(()),
        }
    }
}
