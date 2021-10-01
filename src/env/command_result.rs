//! Utilities for results of commands or processes.

/// A program's exit code.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExitCode(pub i32);
impl ExitCode {
    pub const fn is_success(&self) -> bool {
        self.0 == 0
    }

    pub const fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

/// Encodes whether a command succeeded or failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandResult {
    Success,
    Failure,
}
impl CommandResult {
    pub const fn from_success(success: bool) -> Self {
        if success {
            Self::Success
        } else {
            Self::Failure
        }
    }

    pub const fn is_success(&self) -> bool {
        match self {
            CommandResult::Success => true,
            CommandResult::Failure => false,
        }
    }

    pub const fn is_failure(&self) -> bool {
        !self.is_success()
    }
}
