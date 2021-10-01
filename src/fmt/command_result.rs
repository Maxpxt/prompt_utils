//! Formatting of results of commands or processes.

use crate::{
    env::command_result::{CommandResult, ExitCode},
    style_change, styled_write,
    styling::{Color, Color4Bit, StyleChange, StyledWrite},
    with_style,
};
use std::{fmt, io};

/// Writes a symbol indicating an [exit code][`ExitCode`]'s success status.
///
/// Displays a symbol indicating success or failure,
/// followed or not by the exit code — in the same style as the symbol —
/// depending on `show_code_when` and whether the exit code indicates success.
///
/// `success_symbol` is the symbol to display when the exit code
/// [indicates success][`ExitCode::is_success`].
///
/// `success_style_change` is the [style change][`StyleChange`] to apply
/// when the exit code [indicates success][`ExitCode::is_success`].
///
/// `error_symbol` is the symbol to display when the exit code
/// [indicates failure][`ExitCode::is_failure`].
///
/// `error_style_change` is the [style change][`StyleChange`] to apply
/// when the exit code [indicates failure][`ExitCode::is_failure`].
///
/// `show_code_when` indicates when to display the exit code.
pub fn write_exit_code_symbol(
    writer: &mut (impl StyledWrite + ?Sized),
    exit_code: ExitCode,
    success_symbol: impl fmt::Display,
    success_style_change: StyleChange,
    error_symbol: impl fmt::Display,
    error_style_change: StyleChange,
    show_code_when: When,
) -> io::Result<()> {
    let style_change = if exit_code.is_success() {
        success_style_change
    } else {
        error_style_change
    };
    with_style!(writer, style_change; {
        let show_code = if exit_code.is_success() {
            write!(writer, "{}", success_symbol)?;
            matches!(show_code_when, When::Always)
        } else {
            write!(writer, "{}", error_symbol)?;
            matches!(show_code_when, When::Always | When::OnError)
        };

        if show_code {
            write!(writer, " {}", exit_code.0)?;
        }
    })
}

/// [`write_exit_code_symbol`] with default values for the symbols and their styles.
///
/// This simply calls [`write_exit_code_symbol`] forwarding the parameters
/// and with the default values for the symbols and their styles:
///
/// * `success_symbol`: [`DEFAULT_SUCCESS_SYMBOL`]
/// * `success_style_change`: [`DEFAULT_SUCCESS_STYLE_CHANGE`]
/// * `error_symbol`: [`DEFAULT_ERROR_SYMBOL`]
/// * `error_style_change`: [`DEFAULT_ERROR_STYLE_CHANGE`]
pub fn write_exit_code_symbol_with_defaults(
    writer: &mut (impl StyledWrite + ?Sized),
    exit_code: ExitCode,
    show_code_when: When,
) -> io::Result<()> {
    write_exit_code_symbol(
        writer,
        exit_code,
        DEFAULT_SUCCESS_SYMBOL,
        DEFAULT_SUCCESS_STYLE_CHANGE,
        DEFAULT_ERROR_SYMBOL,
        DEFAULT_ERROR_STYLE_CHANGE,
        show_code_when,
    )
}

/// When to show the exit code in [`write_exit_code_symbol`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum When {
    /// Never show the exit code.
    Never,
    /// Only show the exit code when it [indicates failures][`ExitCode::is_failure`].
    OnError,
    /// Always show the exit code.
    Always,
}

/// Writes a [command result][`CommandResult`] using a symbol for each status.
///
/// Displays a symbol indicating success or failure,
/// followed or not by the exit code — in the same style as the symbol —
/// depending on `show_code_when` and whether the exit code indicates success.
///
/// `success_symbol` is the symbol to display for [`CommandResult::Success`].
///
/// `success_style_change` is the [style change][`StyleChange`]
/// for [`CommandResult::Success`]'s symbol.
///
/// `error_symbol` is the symbol to display for [`CommandResult::Failure`].
///
/// `error_style_change` is the [style change][`StyleChange`]
/// for [`CommandResult::Failure`]'s symbol.
pub fn write_command_result(
    writer: &mut (impl StyledWrite + ?Sized),
    command_result: CommandResult,
    success_symbol: impl fmt::Display,
    success_style_change: StyleChange,
    error_symbol: impl fmt::Display,
    error_style_change: StyleChange,
) -> io::Result<()> {
    if command_result.is_success() {
        styled_write!(writer, success_style_change; "{}", success_symbol)
    } else {
        styled_write!(writer, error_style_change; "{}", error_symbol)
    }
}

/// [`write_command_result`] with default values for the symbols and their styles.
///
/// This simply calls [`write_command_result`] forwarding the parameters
/// and with the default values for the symbols and their styles:
///
/// * `success_symbol`: [`DEFAULT_SUCCESS_SYMBOL`]
/// * `success_style_change`: [`DEFAULT_SUCCESS_STYLE_CHANGE`]
/// * `error_symbol`: [`DEFAULT_ERROR_SYMBOL`]
/// * `error_style_change`: [`DEFAULT_ERROR_STYLE_CHANGE`]
pub fn write_command_result_with_defaults(
    writer: &mut (impl StyledWrite + ?Sized),
    command_result: CommandResult,
) -> io::Result<()> {
    write_command_result(
        writer,
        command_result,
        DEFAULT_SUCCESS_SYMBOL,
        DEFAULT_SUCCESS_STYLE_CHANGE,
        DEFAULT_ERROR_SYMBOL,
        DEFAULT_ERROR_STYLE_CHANGE,
    )
}

pub const DEFAULT_SUCCESS_SYMBOL: char = '✔';
pub const DEFAULT_SUCCESS_STYLE_CHANGE: StyleChange = style_change! {
    foreground: Color::Color4Bit(Color4Bit::BRIGHT_GREEN),
};
pub const DEFAULT_ERROR_SYMBOL: char = '✘';
pub const DEFAULT_ERROR_STYLE_CHANGE: StyleChange = style_change! {
    foreground: Color::Color4Bit(Color4Bit::BRIGHT_RED),
};
