//! Formatting for durations.

use std::{io, time::Duration};

/// A "human-readable" duration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HumanDuration {
    days: u128,
    /// Must be less than 24.
    hours: u8,
    /// Must be less than 60.
    minutes: u8,
    /// Must be less than 60.
    seconds: u8,
    /// Must be less than 1000.
    milliseconds: u16,
    /// Must be less than 1000.
    microseconds: u16,
    /// Must be less than 1000.
    nanoseconds: u16,
}
impl HumanDuration {
    /// Creates a new [`HumanDuration`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee [`HumanDuration`]'s constraints:
    ///
    /// * `hours < 24`
    /// * `minutes < 60`
    /// * `seconds < 60`
    /// * `milliseconds < 1000`
    /// * `microseconds < 1000`
    /// * `nanoseconds < 1000`
    pub unsafe fn new(
        days: u128,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
        microseconds: u16,
        nanoseconds: u16,
    ) -> Self {
        Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        }
    }

    /// Attempts to create a new [`HumanDuration`].
    ///
    /// If all [`HumanDuration`] constraints are met, returns a [`HumanDuration`],
    /// otherwise returns [`None`]. The constraints are:
    ///
    /// * `hours < 24`
    /// * `minutes < 60`
    /// * `seconds < 60`
    /// * `milliseconds < 1000`
    /// * `microseconds < 1000`
    /// * `nanoseconds < 1000`
    pub fn try_new(
        days: u128,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
        microseconds: u16,
        nanoseconds: u16,
    ) -> Option<Self> {
        (matches!(hours, 0..=23)
            && matches!(minutes, 0..=59)
            && matches!(seconds, 0..=59)
            && matches!(milliseconds, 0..=999)
            && matches!(microseconds, 0..=999)
            && matches!(nanoseconds, 0..=999))
        .then(move || Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        })
    }

    /// Creates a [`HumanDuration`] from a [`Duration`].
    pub fn from_duration(duration: Duration) -> Self {
        let nanoseconds = duration.subsec_nanos();
        let (microseconds, nanoseconds) = (nanoseconds / 1000, (nanoseconds % 1000) as u16);
        let (milliseconds, microseconds) = (microseconds / 1000, (microseconds % 1000) as u16);

        let seconds = duration.as_secs();
        let (minutes, seconds) = (seconds / 60, (seconds % 60) as u8);
        let (hours, minutes) = (minutes / 60, (minutes % 60) as u8);
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days: days as u128,
            hours,
            minutes,
            seconds,
            milliseconds: milliseconds as u16,
            microseconds,
            nanoseconds,
        }
    }

    /// Creates a [`HumanDuration`] from a total number of nanoseconds.
    pub fn from_nanoseconds(nanoseconds: u128) -> Self {
        let (microseconds, nanoseconds) = (nanoseconds / 1000, (nanoseconds % 1000) as u16);
        let (milliseconds, microseconds) = (microseconds / 1000, (microseconds % 1000) as u16);
        let (seconds, milliseconds) = (milliseconds / 1000, (milliseconds % 1000) as u16);
        let (minutes, seconds) = (seconds / 60, (seconds % 60) as u8);
        let (hours, minutes) = (minutes / 60, (minutes % 60) as u8);
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        }
    }

    /// Creates a [`HumanDuration`] from a total number of microseconds.
    pub fn from_microseconds(microseconds: u128) -> Self {
        let (milliseconds, microseconds) = (microseconds / 1000, (microseconds % 1000) as u16);
        let (seconds, milliseconds) = (milliseconds / 1000, (milliseconds % 1000) as u16);
        let (minutes, seconds) = (seconds / 60, (seconds % 60) as u8);
        let (hours, minutes) = (minutes / 60, (minutes % 60) as u8);
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds: 0,
        }
    }

    /// Creates a [`HumanDuration`] from a total number of milliseconds.
    pub fn from_milliseconds(milliseconds: u128) -> Self {
        let (seconds, milliseconds) = (milliseconds / 1000, (milliseconds % 1000) as u16);
        let (minutes, seconds) = (seconds / 60, (seconds % 60) as u8);
        let (hours, minutes) = (minutes / 60, (minutes % 60) as u8);
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds: 0,
            nanoseconds: 0,
        }
    }
    /// Creates a [`HumanDuration`] from a total number of seconds.
    pub fn from_seconds(seconds: u128) -> Self {
        let (minutes, seconds) = (seconds / 60, (seconds % 60) as u8);
        let (hours, minutes) = (minutes / 60, (minutes % 60) as u8);
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days,
            hours,
            minutes,
            seconds,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
        }
    }

    /// Creates a [`HumanDuration`] from a total number of minutes.
    pub fn from_minutes(minutes: u128) -> Self {
        let (hours, minutes) = (minutes / 60, (minutes % 60) as u8);
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days,
            hours,
            minutes,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
        }
    }

    /// Creates a [`HumanDuration`] from a total number of hours.
    pub fn from_hours(hours: u128) -> Self {
        let (days, hours) = (hours / 24, (hours % 24) as u8);

        Self {
            days,
            hours,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
        }
    }

    /// Creates a [`HumanDuration`] from a total number of days.
    pub fn from_days(days: u128) -> Self {
        Self {
            days,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
        }
    }

    /// The days component.
    pub fn days(&self) -> u128 {
        self.days
    }

    /// The hours component.
    pub fn hours(&self) -> u8 {
        self.hours
    }

    /// The minutes component.
    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    /// The seconds component.
    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    /// The milliseconds component.
    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    /// The microseconds component.
    pub fn microseconds(&self) -> u16 {
        self.microseconds
    }

    /// The nanoseconds component.
    pub fn nanoseconds(&self) -> u16 {
        self.nanoseconds
    }

    /// The truncation of this [`HumanDuration`] to days precision.
    pub fn truncated_to_days(&self) -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
            ..*self
        }
    }

    /// The truncation of this [`HumanDuration`] to hours precision.
    pub fn truncated_to_hours(&self) -> Self {
        Self {
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
            ..*self
        }
    }

    /// The truncation of this [`HumanDuration`] to minutes precision.
    pub fn truncated_to_minutes(&self) -> Self {
        Self {
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
            ..*self
        }
    }

    /// The truncation of this [`HumanDuration`] to seconds precision.
    pub fn truncated_to_seconds(&self) -> Self {
        Self {
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
            ..*self
        }
    }

    /// The truncation of this [`HumanDuration`] to milliseconds precision.
    pub fn truncated_to_milliseconds(&self) -> Self {
        Self {
            microseconds: 0,
            nanoseconds: 0,
            ..*self
        }
    }
    /// The truncation of this [`HumanDuration`] to microseconds precision.
    pub fn truncated_to_microseconds(&self) -> Self {
        Self {
            nanoseconds: 0,
            ..*self
        }
    }
}

/// Write all components of a [`HumanDuration`].
pub fn write_all(
    writer: &mut (impl io::Write + ?Sized),
    duration: HumanDuration,
) -> io::Result<()> {
    write!(
        writer,
        "{}d {}h {}m {}s {}ms {}µs {}ns",
        duration.days,
        duration.hours,
        duration.minutes,
        duration.seconds,
        duration.milliseconds,
        duration.microseconds,
        duration.nanoseconds,
    )
}

/// Write all nonzero components of a [`HumanDuration`].
pub fn write_nonzero(
    writer: &mut (impl io::Write + ?Sized),
    duration: HumanDuration,
) -> io::Result<()> {
    let mut is_first = true;

    macro_rules! write_part {
        ($format:expr, $value:expr) => {
            if $value != 0 {
                if !is_first {
                    write!(writer, " ")?;
                }

                write!(writer, $format, $value)?;

                #[allow(unused_assignments)]
                {
                    is_first = false;
                }
            }
        };
    }

    write_part!("{}d", duration.days);
    write_part!("{}h", duration.hours);
    write_part!("{}m", duration.minutes);
    write_part!("{}s", duration.seconds);
    write_part!("{}ms", duration.milliseconds);
    write_part!("{}µs", duration.microseconds);
    write_part!("{}ns", duration.nanoseconds);

    Ok(())
}

/// Write some components of a [`HumanDuration`].
#[allow(clippy::too_many_arguments)]
pub fn write_some(
    writer: &mut (impl io::Write + ?Sized),
    duration: HumanDuration,
    days: bool,
    hours: bool,
    minutes: bool,
    seconds: bool,
    milliseconds: bool,
    microseconds: bool,
    nanoseconds: bool,
) -> io::Result<()> {
    let mut is_first = true;

    macro_rules! write_part {
        ($condition:expr, $format:expr, $value:expr) => {
            if $condition {
                if !is_first {
                    write!(writer, " ")?;
                }

                write!(writer, $format, $value)?;

                #[allow(unused_assignments)]
                {
                    is_first = false;
                }
            }
        };
    }

    write_part!(days, "{}d", duration.days);
    write_part!(hours, "{}h", duration.hours);
    write_part!(minutes, "{}m", duration.minutes);
    write_part!(seconds, "{}s", duration.seconds);
    write_part!(milliseconds, "{}ms", duration.milliseconds);
    write_part!(microseconds, "{}µs", duration.microseconds);
    write_part!(nanoseconds, "{}ns", duration.nanoseconds);

    Ok(())
}

/// Write a [`HumanDuration`] starting from the most significant nonzero component.
pub fn write_skip_high_zeros(
    writer: &mut (impl io::Write + ?Sized),
    duration: HumanDuration,
) -> io::Result<()> {
    let mut is_first = true;

    macro_rules! write_part {
        ($format:expr, $value:expr) => {
            if $value != 0 || !is_first {
                if !is_first {
                    write!(writer, " ")?;
                }

                write!(writer, $format, $value)?;

                #[allow(unused_assignments)]
                {
                    is_first = false;
                }
            }
        };
    }

    write_part!("{}d", duration.days);
    write_part!("{}h", duration.hours);
    write_part!("{}m", duration.minutes);
    write_part!("{}s", duration.seconds);
    write_part!("{}ms", duration.milliseconds);
    write_part!("{}µs", duration.microseconds);
    write_part!("{}ns", duration.nanoseconds);

    Ok(())
}

/// Write a [`HumanDuration`] up to the least significant nonzero component.
pub fn write_skip_low_zeros(
    writer: &mut (impl io::Write + ?Sized),
    duration: HumanDuration,
) -> io::Result<()> {
    let mut write_count: u32 = if duration.nanoseconds != 0 {
        7
    } else if duration.microseconds != 0 {
        6
    } else if duration.milliseconds != 0 {
        5
    } else if duration.seconds != 0 {
        4
    } else if duration.minutes != 0 {
        3
    } else if duration.hours != 0 {
        2
    } else if duration.days != 0 {
        1
    } else {
        0
    };

    macro_rules! write_part {
        ($format:expr, $value:expr) => {
            if write_count >= 1 {
                write!(writer, $format, $value)?;

                #[allow(unused_assignments)]
                {
                    write_count -= 1;
                }
            } else {
                return Ok(());
            }
        };
    }

    write_part!("{}d", duration.days);
    write_part!(" {}h", duration.hours);
    write_part!(" {}m", duration.minutes);
    write_part!(" {}s", duration.seconds);
    write_part!(" {}ms", duration.milliseconds);
    write_part!(" {}µs", duration.microseconds);
    write_part!(" {}ns", duration.nanoseconds);

    Ok(())
}

/// Write a [`HumanDuration`] from the most significant nonzero component to least significant one.
pub fn write_skip_high_and_low_zeros(
    writer: &mut (impl io::Write + ?Sized),
    duration: HumanDuration,
) -> io::Result<()> {
    let mut is_first = true;

    let mut write_count: u32 = if duration.nanoseconds != 0 {
        7
    } else if duration.microseconds != 0 {
        6
    } else if duration.milliseconds != 0 {
        5
    } else if duration.seconds != 0 {
        4
    } else if duration.minutes != 0 {
        3
    } else if duration.hours != 0 {
        2
    } else if duration.days != 0 {
        1
    } else {
        0
    };

    macro_rules! write_part {
        ($format:expr, $value:expr) => {
            if write_count >= 1 {
                if $value != 0 || !is_first {
                    if !is_first {
                        write!(writer, " ")?;
                    }

                    write!(writer, $format, $value)?;

                    #[allow(unused_assignments)]
                    {
                        is_first = false;
                    }
                }

                #[allow(unused_assignments)]
                {
                    write_count -= 1;
                }
            } else {
                return Ok(());
            }
        };
    }

    write_part!("{}d", duration.days);
    write_part!("{}h", duration.hours);
    write_part!("{}m", duration.minutes);
    write_part!("{}s", duration.seconds);
    write_part!("{}ms", duration.milliseconds);
    write_part!("{}µs", duration.microseconds);
    write_part!("{}ns", duration.nanoseconds);

    Ok(())
}
