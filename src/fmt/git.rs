//! Formatting for information about a [git] repository.
//!
//! [git]: https://git-scm.com/

use crate::{
    env::git::{AheadBehind, ChangeSummary, Head, StatusSummary},
    styling::StyledWrite,
};
use std::io;

/// Writes a short representation of a [`Head`].
///
/// Writes the name (when [`Branch`][`Head::Branch`] or [`Unborn`][`Head::Unborn`])
/// or short hash (when [`Commit`][`Head::Commit`]) of the [`Head`]'s target
/// preceded by a symbol indicating the [`Head`]'s state.
/// When applicable and present, the [ahead and behind upstream count][`Head::Branch::upstream`]
/// then follows, in the format of [`write_ahead_behind`].
pub fn write_head(writer: &mut (impl StyledWrite + ?Sized), head: &Head) -> io::Result<()> {
    match head {
        Head::Unborn { target } => write!(
            writer,
            "○{}",
            target.strip_prefix("refs/heads/").unwrap_or(target),
        ),
        Head::Branch { name, upstream } => {
            write!(writer, "{}", name)?;
            if let Ok(Some(upstream)) = upstream {
                write!(writer, " ")?;
                write_ahead_behind(writer, upstream)?;
            }
            Ok(())
        }
        Head::Commit(id) => {
            let id_string = id.to_string();
            write!(writer, "◉{}", &id_string[..id_string.len().min(6)])
        }
    }
}

/// Writes a short representation of an [`AheadBehind`].
///
/// [`ahead`] and [`behind`] are written preceded by `↑` and `↓`, respectively,
/// and omitted when zero.
/// When both [`ahead`] and [`behind`] are zero, `≡` is written,
/// signifying that the branch and its [upstream][upstream branch] point to the same commit.
///
/// [`ahead`]: `AheadBehind::ahead`
/// [`behind`]: `AheadBehind::behind`
/// [upstream branch]: https://git-scm.com/docs/gitglossary#def_upstream_branch
pub fn write_ahead_behind(
    writer: &mut (impl StyledWrite + ?Sized),
    ahead_behind: &AheadBehind,
) -> io::Result<()> {
    if ahead_behind.ahead == 0 && ahead_behind.behind == 0 {
        write!(writer, "≡")?;
    } else {
        let mut is_preceded = false;
        if ahead_behind.ahead != 0 {
            write!(writer, "↑{}", ahead_behind.ahead)?;
            is_preceded = true;
        }
        if ahead_behind.behind != 0 {
            if is_preceded {
                write!(writer, " ")?;
            }
            write!(writer, "↓{}", ahead_behind.behind)?;
        }
    }
    Ok(())
}

/// Writes a short representation of a [`StatusSummary`].
///
/// The [working tree][`StatusSummary::working_tree`] and [staging area][`StatusSummary::staging`]
/// changes are, in that order, written following the format of [`write_change_summary`]
/// and separated by a vertical bar `|`.
/// When it is not zero, the [count of files with merge conflicts][`StatusSummary::conflicted`]
/// follows preceded by an exclamation mark `!`.
pub fn write_status_summary(
    writer: &mut (impl StyledWrite + ?Sized),
    status: &StatusSummary,
) -> io::Result<()> {
    let mut is_preceded = false;
    if status.staging.any_changes() {
        write_change_summary(writer, &status.staging)?;
        is_preceded = true;
    }
    if status.working_tree.any_changes() {
        if is_preceded {
            write!(writer, " ")?;
        }
        write!(writer, "| ")?;
        write_change_summary(writer, &status.working_tree)?;
        is_preceded = true;
    }
    if status.conflicted != 0 {
        if is_preceded {
            write!(writer, " ")?;
        }
        write!(writer, "!{}", status.conflicted)?;
    }
    Ok(())
}

/// Writes a short representation of a [`ChangeSummary`].
///
/// The [added][`ChangeSummary::added`], [modified][`ChangeSummary::modified`],
/// and [deleted][`ChangeSummary::deleted`] counts are, in that order, written
/// preceded by `+`, `~`, and `-`, respectively.
/// Any of these counts that are zero are omitted.
pub fn write_change_summary(
    writer: &mut (impl StyledWrite + ?Sized),
    changes: &ChangeSummary,
) -> io::Result<()> {
    let mut is_preceded = false;
    if changes.added != 0 {
        write!(writer, "+{}", changes.added)?;
        is_preceded = true;
    }
    if changes.modified != 0 {
        if is_preceded {
            write!(writer, " ")?;
        }
        write!(writer, "~{}", changes.modified)?;
        is_preceded = true;
    }
    if changes.deleted != 0 {
        if is_preceded {
            write!(writer, " ")?;
        }
        write!(writer, "-{}", changes.deleted)?;
    }
    Ok(())
}
