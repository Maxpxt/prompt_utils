//! Interfaces for writing styled text.

use bitflags::bitflags;
use std::io;

/// Expands to a [`StyleChange`] that [sets][`Change::SetTo`] the indicated fields
/// and [keeps][`Change::Keep`] the rest.
///
/// This macro accepts a comma separated list of `field: value`s, like the struct creation syntax.
/// It expands to a [`StyleChange`] where each indicated `field`
/// is set to [`Change::SetTo(value)`][`Change::SetTo`], with the corresponding `value`,
/// and the the rest is set to [`Change::Keep`].
///
/// # Examples
///
/// ```rust
/// # use prompt_utils::{
/// #     style_change,
/// #     styling::{Change, StyleChange},
/// # };
/// #
/// let style_change = style_change! { bold: true, italic: true };
/// assert_eq!(
///     style_change,
///     StyleChange {
///         bold: Change::SetTo(true),
///         italic: Change::SetTo(true),
///         ..Default::default()
///     }
/// );
/// ```
#[macro_export]
macro_rules! style_change {
    { $($field:ident : $value:expr),* $(,)? } => {
        $crate::styling::StyleChange {
            $($field : $crate::styling::Change::SetTo($value),)*
            ..$crate::styling::StyleChange::KEEP
        }
    };
}

/// Temporarily changes the style of a [`StyledWrite`].
///
/// This macro expects three arguments:
///
/// * The writer, which is an expression that resolves into a
///   [`&mut impl StyledWrite`][`StyledWrite`].
///   Note that the macro may expand to execute the writer expression multiple times,
///   so only simple expressions should be used.
/// * The style change to be temporarily applied.
///   This can either be an expression resolving into a [`StyleChange`]
///   or a list of `field: value`s following the syntax and semantics of [`style_change`].
/// * The code to be executed while the style change is in effect.
///   This argument is separated by the previous using a semicolon `;`.
///
/// The macro's return value is an [`std::io::Result`].
/// When an error occurs while interacting with the writer,
/// it is bubbled up in the [`Err`][`std::result::Result::Err`] variant.
/// When no error occurs, the [`Ok`][`std::result::Result::Ok`] variant is returned with the
/// result of the executed code (the third argument of the macro).
///
/// The style change is [reverted][`StyleChange::reverting_to`] after the code is executed.
#[macro_export]
macro_rules! with_style {
    ($writer:expr $(, $field:ident : $value:expr)* $(,)? ; $code:expr $(,)?) => {
        $crate::with_style!($writer, $crate::style_change! { $($field : $value,)+ }; $code)
    };
    ($writer:expr, $style_change:expr; $code:expr $(,)?) => {{
        let style_change = $style_change;
        #[allow(clippy::unnecessary_mut_passed)]
        let revert_style_change =
            style_change.reverting_to($crate::styling::StyledWrite::style($writer));
        match $crate::styling::StyledWrite::change_style($writer, style_change) {
            Err(err) => Err(err),
            Ok(()) => {
                let result = $code;
                match $crate::styling::StyledWrite::change_style($writer, revert_style_change) {
                    Ok(()) => Ok(result),
                    Err(err) => Err(err),
                }
            }
        }
    }};
}

/// Temporarily changes the style of a [`StyledWrite`]
/// and finishes by [resetting][`StyledWrite::reset_style`] the style.
///
/// This macro expects three arguments:
///
/// * The writer, which is an expression that resolves into a
///   [`&mut impl StyledWrite`][`StyledWrite`].
///   Note that the macro may expand to execute the writer expression multiple times,
///   so only simple expressions should be used.
/// * The style change to be temporarily applied.
///   This can either be an expression resolving into a [`StyleChange`]
///   or a list of `field: value`s following the syntax and semantics of [`style_change`].
/// * The code to be executed while the style change is in effect.
///   This argument is separated by the previous using a semicolon `;`.
///
/// The macro's return value is an [`std::io::Result`].
/// When an error occurs while interacting with the writer,
/// it is bubbled up in the [`Err`][`std::result::Result::Err`] variant.
/// When no error occurs, the [`Ok`][`std::result::Result::Ok`] variant is returned with the
/// result of the executed code (the third argument of the macro).
///
/// The style is [reset][`StyledWrite::reset_style`] after the code is executed.
#[macro_export]
macro_rules! with_style_then_reset {
    ($writer:expr $(, $field:ident : $value:expr)* $(,)? ; $code:expr $(,)?) => {
        $crate::with_style_then_reset!(
            $writer, $crate::style_change! { $($field : $value,)+ };
            $code
        )
    };
    ($writer:expr, $style_change:expr; $code:expr $(,)?) => {{
        match $crate::styling::StyledWrite::change_style($writer, $style_change) {
            Err(err) => Err(err),
            Ok(()) => {
                let result = $code;
                match $crate::styling::StyledWrite::reset_style($writer) {
                    Ok(()) => Ok(result),
                    Err(err) => Err(err),
                }
            }
        }
    }};
}

/// Temporarily [swaps the foreground and background colors][`StyledWrite::swap_colors`]
/// of a [`StyledWrite`].
///
/// This macro expects two arguments:
///
/// * The writer, which is an expression that resolves into a
///   [`&mut impl StyledWrite`][`StyledWrite`].
///   Note that the macro may expand to execute the writer expression multiple times,
///   so only simple expressions should be used.
/// * The code to be executed while the color swap is in effect.
///   This argument is separated by the previous using a semicolon `;`.
///
/// The macro's return value is an [`std::io::Result`].
/// When an error occurs while interacting with the writer,
/// it is bubbled up in the [`Err`][`std::result::Result::Err`] variant.
/// When no error occurs, the [`Ok`][`std::result::Result::Ok`] variant is returned with the
/// result of the executed code (the second argument of the macro).
///
/// The color swap is reverted after the code is executed.
#[macro_export]
macro_rules! with_swapped_colors {
    ($writer:expr; $code:expr $(,)?) => {{
        match $crate::styling::StyledWrite::swap_colors($writer) {
            Err(err) => Err(err),
            Ok(()) => {
                let result = $code;
                match $crate::styling::StyledWrite::swap_colors($writer) {
                    Ok(()) => Ok(result),
                    Err(err) => Err(err),
                }
            }
        }
    }};
}

/// Writes styled formatted data using a [`StyledWrite`].
///
/// The syntax of this macro is similar to that of [`write!`][`std::write!`],
/// but two arguments are required preceding the format string separated from it by a semicolon `;`:
///
/// * The writer, which is an expression that resolves into a
///   [`&mut impl StyledWrite`][`StyledWrite`].
///   Note that the macro may expand to execute the writer expression multiple times,
///   so only simple expressions should be used.
/// * The style change to be temporarily applied.
///   This can either be an expression resolving into a [`StyleChange`]
///   or a list of `field: value`s following the syntax and semantics of [`style_change`].
///
/// Note that the style change is only temporarily applied, i.e.,
/// it is [reverted][`StyleChange::reverting_to`] after the formatted data is written.
#[macro_export]
macro_rules! styled_write {
    ($writer:expr $(, $field:ident : $value:expr)* $(,)? ; $($args:tt)*) => {
        // TODO: When [`result_flattening`](https://github.com/rust-lang/rust/issues/70142)
        // is stabilized, replace `.and_then(std::convert::identity)` by `.flatten()`.
        $crate::with_style!($writer $(, $field : $value)* ; std::write!($writer, $($args)*))
            .and_then(std::convert::identity)
    };
    ($writer:expr, $style_change:expr; $($args:tt)*) => {
        // TODO: When [`result_flattening`](https://github.com/rust-lang/rust/issues/70142)
        // is stabilized, replace `.and_then(std::convert::identity)` by `.flatten()`.
        $crate::with_style!($writer, $style_change; std::write!($writer, $($args)*))
            .and_then(std::convert::identity)
    };
}

/// Writes styled formatted data using a [`StyledWrite`]
/// and finishes by [resetting][`StyledWrite::reset_style`] the style.
///
/// The syntax of this macro is similar to that of [`write!`][`std::write!`],
/// but two arguments are required preceding the format string separated from it by a semicolon `;`:
///
/// * The writer, which is an expression that resolves into a
///   [`&mut impl StyledWrite`][`StyledWrite`].
///   Note that the macro may expand to execute the writer expression multiple times,
///   so only simple expressions should be used.
/// * The style change to be temporarily applied.
///   This can either be an expression resolving into a [`StyleChange`]
///   or a list of `field: value`s following the syntax and semantics of [`style_change`].
///
/// Note that the style change is only temporarily applied,
/// with the style being [reset][`StyledWrite::reset_style`] after the formatted data is written.
#[macro_export]
macro_rules! styled_write_then_reset {
    ($writer:expr $(, $field:ident : $value:expr)* $(,)? ; $($args:tt)*) => {
        // TODO: When [`result_flattening`](https://github.com/rust-lang/rust/issues/70142)
        // is stabilized, replace `.and_then(std::convert::identity)` by `.flatten()`.
        $crate::with_style_then_reset!(
            $writer $(, $field : $value)* ; std::write!($writer, $($args)*)
        ).and_then(std::convert::identity)
    };
    ($writer:expr, $style_change:expr; $($args:tt)*) => {
        // TODO: When [`result_flattening`](https://github.com/rust-lang/rust/issues/70142)
        // is stabilized, replace `.and_then(std::convert::identity)` by `.flatten()`.
        $crate::with_style_then_reset!($writer, $style_change; std::write!($writer, $($args)*))
            .and_then(std::convert::identity)
    };
}

/// Writes styled formatted data using a [`StyledWrite`]
/// with foreground and background colors swapped.
///
/// The syntax of this macro is similar to that of [`write!`][`std::write!`],
/// but the writer (the first argument) is an expression that resolves into a
/// [`&mut impl StyledWrite`][`StyledWrite`]
/// and is separated from the format string by a semicolon `;`.
///
/// Note that the macro may expand to execute the writer expression multiple times,
/// so only simple expressions should be used.
///
/// Note also that the color swap is only temporarily applied, i.e.,
/// it is reverted after the formatted data is written.
#[macro_export]
macro_rules! swapped_colors_write {
    ($writer:expr; $($args:tt)*) => {
        // TODO: When [`result_flattening`](https://github.com/rust-lang/rust/issues/70142)
        // is stabilized, replace `.and_then(std::convert::identity)` by `.flatten()`.
        $crate::with_swapped_colors!($writer; std::write!($writer, $($args)*))
            .and_then(std::convert::identity)
    };
}

/// Trait for a text writer capable of styling.
pub trait StyledWrite: io::Write {
    /// Gets the current text style.
    fn style(&self) -> &Style;

    /// Changes some (or all) of the text style properties for future writes.
    fn change_style(&mut self, change: StyleChange) -> io::Result<()>;

    /// Resets the text style for future writes.
    fn reset_style(&mut self) -> io::Result<()> {
        self.change_style(StyleChange::RESET)
    }

    /// Swaps the foreground and background colors of the text style for future writes.
    fn swap_colors(&mut self) -> io::Result<()> {
        let style = self.style();
        let style_change = StyleChange {
            foreground: Change::SetTo(style.background),
            background: Change::SetTo(style.foreground),
            ..Default::default()
        };
        self.change_style(style_change)
    }
}

/// Encodes a text style.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub dim: bool,
    pub underline: bool,
    pub italic: bool,
    pub blink: bool,
    pub strike: bool,
}
impl Default for Style {
    fn default() -> Style {
        Style {
            foreground: Color::Unset,
            background: Color::Unset,
            bold: false,
            dim: false,
            underline: false,
            italic: false,
            blink: false,
            strike: false,
        }
    }
}
impl Style {
    /// Swaps the foreground and background colors.
    pub fn swap_colors(&mut self) {
        std::mem::swap(&mut self.foreground, &mut self.background)
    }

    /// A clone of this style with the foreground and background colors swapped.
    pub const fn colors_swapped(self) -> Self {
        Self {
            foreground: self.background,
            background: self.foreground,
            ..self
        }
    }
}

/// Encodes a change of some (or all) properties of a text style.
///
/// The [`style_change`] macro provides a concise syntax for creating [`StyleChange`]s
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct StyleChange {
    pub foreground: Change<Color>,
    pub background: Change<Color>,
    pub bold: Change<bool>,
    pub dim: Change<bool>,
    pub underline: Change<bool>,
    pub italic: Change<bool>,
    pub blink: Change<bool>,
    pub strike: Change<bool>,
}
impl StyleChange {
    /// The [`StyleChange`] that keeps all attributes unchanged.
    pub const KEEP: Self = Self {
        foreground: Change::Keep,
        background: Change::Keep,
        bold: Change::Keep,
        dim: Change::Keep,
        underline: Change::Keep,
        italic: Change::Keep,
        blink: Change::Keep,
        strike: Change::Keep,
    };

    /// The [`StyleChange`] that resets all attributes.
    pub const RESET: Self = Self {
        foreground: Change::SetTo(Color::Unset),
        background: Change::SetTo(Color::Unset),
        bold: Change::SetTo(false),
        dim: Change::SetTo(false),
        underline: Change::SetTo(false),
        italic: Change::SetTo(false),
        blink: Change::SetTo(false),
        strike: Change::SetTo(false),
    };

    /// A [`StyleChange`] that sets the style to `style`.
    pub const fn setting_to(style: &Style) -> Self {
        Self {
            foreground: Change::SetTo(style.foreground),
            background: Change::SetTo(style.background),
            bold: Change::SetTo(style.bold),
            dim: Change::SetTo(style.dim),
            underline: Change::SetTo(style.underline),
            italic: Change::SetTo(style.italic),
            blink: Change::SetTo(style.blink),
            strike: Change::SetTo(style.strike),
        }
    }

    /// The [`Style`] resulting from applying the changes indicated by `self` to `style`.
    ///
    ///  # See Also
    ///
    /// [`Change::apply_to`]
    pub const fn apply_to(&self, style: &Style) -> Style {
        Style {
            foreground: match self.foreground {
                Change::Keep => style.foreground,
                Change::SetTo(foreground) => foreground,
            },
            background: match self.background {
                Change::Keep => style.background,
                Change::SetTo(background) => background,
            },
            bold: match self.bold {
                Change::Keep => style.bold,
                Change::SetTo(bold) => bold,
            },
            dim: match self.dim {
                Change::Keep => style.dim,
                Change::SetTo(dim) => dim,
            },
            italic: match self.italic {
                Change::Keep => style.italic,
                Change::SetTo(italic) => italic,
            },
            underline: match self.underline {
                Change::Keep => style.underline,
                Change::SetTo(underline) => underline,
            },
            blink: match self.blink {
                Change::Keep => style.blink,
                Change::SetTo(blink) => blink,
            },
            strike: match self.strike {
                Change::Keep => style.strike,
                Change::SetTo(strike) => strike,
            },
        }
    }

    /// The [`StyleChange`] that reverts `self`.
    ///
    /// Assuming that `previous` was the style before `self` was applied,
    /// this function returns the [`StyleChange`] that will revert
    /// the change made by applying `self`.
    ///
    /// # Note
    ///
    /// If another style change is applied after `self` and before this method's result,
    /// the final style may not be `previous`.
    ///
    /// # See Also
    ///
    /// [`Change::reverting_to`]
    pub fn reverting_to(&self, previous: &Style) -> Self {
        Self {
            foreground: self.foreground.reverting_to(previous.foreground),
            background: self.background.reverting_to(previous.background),
            bold: self.bold.reverting_to(previous.bold),
            dim: self.dim.reverting_to(previous.dim),
            underline: self.underline.reverting_to(previous.underline),
            italic: self.italic.reverting_to(previous.italic),
            blink: self.blink.reverting_to(previous.blink),
            strike: self.strike.reverting_to(previous.strike),
        }
    }

    /// Tells whether `self` encodes any change, i.e.,
    /// if any of its fields is not [`Change::Keep`].
    pub const fn any(&self) -> bool {
        !matches!(
            self,
            Self {
                foreground: Change::Keep,
                background: Change::Keep,
                bold: Change::Keep,
                dim: Change::Keep,
                underline: Change::Keep,
                italic: Change::Keep,
                blink: Change::Keep,
                strike: Change::Keep,
            }
        )
    }
}

/// A command for the change of some value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Change<T> {
    /// Tells to keep the current value.
    Keep,
    /// Tells to change the value.
    SetTo(T),
}
impl<T> Default for Change<T> {
    fn default() -> Self {
        Change::Keep
    }
}
impl<T> Change<T> {
    /// The result of applying the change indicated by `self` to `value`.
    pub fn apply_to(self, value: T) -> T {
        match self {
            Change::Keep => value,
            Change::SetTo(value) => value,
        }
    }

    /// The [`Change`] that reverts `self`.
    ///
    /// Assuming that `previous` was the value before `self` was applied,
    /// this function returns the [`Change`] that will revert the change made by applying `self`.
    ///
    /// # Note
    ///
    /// If another change is applied after `self` and before this method's result,
    /// the final value may not be `previous`.
    pub fn reverting_to(&self, previous: T) -> Change<T> {
        match self {
            Change::Keep => Change::Keep,
            Change::SetTo(_) => Change::SetTo(previous),
        }
    }
}

/// Text background or foreground color.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Unset,
    Color4Bit(Color4Bit),
    ANSI256(u8),
    RGB(u8, u8, u8),
}
impl Default for Color {
    fn default() -> Self {
        Color::Unset
    }
}

bitflags! {
    /// 4-bit colors
    ///
    /// One bit each for the red, green and blue components
    /// ([`RED_BIT`](`Self::RED_BIT`),
    /// [`GREEN_BIT`](`Self::GREEN_BIT`),
    /// [`BLUE_BIT`](`Self::BLUE_BIT`))
    /// and one bit for selecting between the bright and not bright (dark) variants
    /// ([`BRIGHT_BIT`](`Self::BRIGHT_BIT`)).
    /// All 16 possible colors can be obtained by combining these bits.
    pub struct Color4Bit: u8 {
        const BRIGHT_BIT = 0b1000;

        const RED_BIT = 0b0001;
        const GREEN_BIT = 0b0010;
        const BLUE_BIT = 0b0100;

        const COLOR_MASK = Self::RED_BIT.bits | Self::GREEN_BIT.bits | Self::BLUE_BIT.bits;

        const BLACK = Self::empty().bits;
        const DARK_RED = Self::RED_BIT.bits;
        const DARK_YELLOW = Self::RED_BIT.bits | Self::GREEN_BIT.bits;
        const DARK_GREEN = Self::GREEN_BIT.bits;
        const DARK_CYAN = Self::GREEN_BIT.bits | Self::BLUE_BIT.bits;
        const DARK_BLUE = Self::BLUE_BIT.bits;
        const DARK_MAGENTA = Self::BLUE_BIT.bits | Self::RED_BIT.bits;
        const DARK_GRAY = Self::RED_BIT.bits | Self::GREEN_BIT.bits | Self::BLUE_BIT.bits;
        const BRIGHT_GRAY = Self::BRIGHT_BIT.bits;
        const BRIGHT_RED = Self::BRIGHT_BIT.bits | Self::RED_BIT.bits;
        const BRIGHT_YELLOW = Self::BRIGHT_BIT.bits | Self::RED_BIT.bits | Self::GREEN_BIT.bits;
        const BRIGHT_GREEN = Self::BRIGHT_BIT.bits | Self::GREEN_BIT.bits;
        const BRIGHT_CYAN = Self::BRIGHT_BIT.bits | Self::GREEN_BIT.bits | Self::BLUE_BIT.bits;
        const BRIGHT_BLUE = Self::BRIGHT_BIT.bits | Self::BLUE_BIT.bits;
        const BRIGHT_MAGENTA = Self::BRIGHT_BIT.bits | Self::BLUE_BIT.bits | Self::RED_BIT.bits;
        const WHITE = Self::BRIGHT_BIT.bits
            | Self::RED_BIT.bits
            | Self::GREEN_BIT.bits
            | Self::BLUE_BIT.bits;
    }
}
impl Color4Bit {
    /// The code of this color in the 256 colors ANSI escape sequences
    /// (`ESC[38;5;{code}m` and `ESC[48;5;{code}m`).
    pub const fn to_ansi_256(&self) -> u8 {
        self.intersection(Self::all()).bits
    }
}
