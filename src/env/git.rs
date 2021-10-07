//! Utilities for querying and representing information about a [git] repository.
//!
//! [git]: https://git-scm.com/

use git2::{
    Branch, Error, ErrorClass, ErrorCode, Oid, Repository, RepositoryOpenFlags, Status,
    StatusOptions,
};
use std::{path::Path, str};

/// Finds and [opens][`Repository::open`] a repository.
///
/// The search is done as git would from `dir`.
pub fn open_repo(dir: &Path) -> Result<Repository, Error> {
    Repository::open_ext(dir, RepositoryOpenFlags::FROM_ENV, None::<&Path>)
}

/// Gets the information about a repository's [HEAD].
///
/// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
pub fn query_head(repo: &Repository) -> Result<Head, Error> {
    Head::from_repo(repo)
}

/// Gets the [summary][`StatusSummary`] of a repository's [status].
///
/// [status]: https://git-scm.com/docs/git-status
pub fn query_status_summary(repo: &Repository) -> Result<StatusSummary, Error> {
    StatusSummary::from_repo(repo)
}

/// Gets the number of [stashes][stash] in a repository's.
///
/// [stash]: https://git-scm.com/docs/gitglossary#def_stash
pub fn query_stash_count(repo: &mut Repository) -> Result<usize, Error> {
    let mut count = 0;
    repo.stash_foreach(|_, _, _| {
        count += 1;
        true
    })?;
    Ok(count)
}

/// Information about a repository's [HEAD].
///
/// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
#[derive(Debug, PartialEq)]
pub enum Head {
    /// [HEAD] points to an existing branch.
    ///
    /// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
    Branch {
        /// The name of the branch.
        name: String,
        /// The count of how many commits the branch is ahead and behind its
        /// [upstream][upstream branch].
        ///
        /// This is [`None`] if the branch has no upstream,
        /// and [`Err`] if an error occurs while getting the information.
        ///
        /// [upstream branch]: https://git-scm.com/docs/gitglossary#def_upstream_branch
        upstream: Result<Option<AheadBehind>, Error>,
    },
    /// HEAD is in [detached][detached HEAD] state.
    ///
    /// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
    /// [detached HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
    Commit(Oid),
    /// [HEAD] points to a nonexisting target.
    ///
    /// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
    Unborn {
        /// The target (usually a branch) of the [HEAD] [symbolic reference][symref].
        ///
        /// **Tip:** [`target.strip_prefix("refs/heads/")`][`str::strip_prefix`]
        /// returns the name of the branch `target` points to.
        ///
        /// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
        /// [symref]: https://git-scm.com/docs/gitglossary#def_symref
        target: String,
    },
}
impl Head {
    /// Gets the information about a repository's [HEAD].
    ///
    /// [HEAD]: https://git-scm.com/docs/gitglossary#def_HEAD
    pub fn from_repo(repo: &Repository) -> Result<Self, Error> {
        match repo.find_reference("HEAD") {
            Ok(head) => match head.symbolic_target_bytes() {
                Some(target) => match str::from_utf8(target) {
                    Ok(target) => match repo.find_reference(target) {
                        Ok(reference) => {
                            let branch = Branch::wrap(reference);
                            Ok(Head::Branch {
                                name: String::from_utf8_lossy(branch.name_bytes()?).into_owned(),
                                upstream: AheadBehind::from_branch(repo, branch),
                            })
                        }
                        Err(err) if err.code() == ErrorCode::NotFound => Ok(Head::Unborn {
                            target: String::from(target),
                        }),
                        Err(err) => Err(err),
                    },
                    Err(_) => Err(Error::new(
                        ErrorCode::GenericError,
                        ErrorClass::Reference,
                        "could not decode HEAD's symbolic target.",
                    )),
                },
                None => match head.target() {
                    Some(target) => {
                        let commit = repo.find_commit(target)?;
                        Ok(Head::Commit(commit.id()))
                    }
                    None => Err(Error::new(
                        ErrorCode::GenericError,
                        ErrorClass::Reference,
                        "HEAD is neither direct nor symbolic.",
                    )),
                },
            },
            Err(err) => Err(err),
        }
    }
}

/// Counts of how many commits a branch is ahead and behind its [upstream][upstream branch].
///
/// [upstream branch]: https://git-scm.com/docs/gitglossary#def_upstream_branch
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AheadBehind {
    pub ahead: usize,
    pub behind: usize,
}
impl AheadBehind {
    /// Gets the count of how many commits a branch is ahead and behind its
    /// [upstream][upstream branch].
    ///
    /// [upstream branch]: https://git-scm.com/docs/gitglossary#def_upstream_branch
    pub fn from_branch(repo: &Repository, branch: Branch) -> Result<Option<AheadBehind>, Error> {
        match branch.upstream() {
            Ok(upstream) => {
                let (ahead, behind) = repo.graph_ahead_behind(
                    branch.get().peel_to_commit()?.id(),
                    upstream.get().peel_to_commit()?.id(),
                )?;
                Ok(Some(AheadBehind { ahead, behind }))
            }
            Err(err) if err.code() == ErrorCode::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }
}

/// A summary of a repository's [status].
///
/// [status]: https://git-scm.com/docs/git-status
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct StatusSummary {
    /// The [summary][`ChangeSummary`] of the changes in the [working tree].
    ///
    /// [working tree]: https://git-scm.com/docs/gitglossary#def_working_tree
    pub working_tree: ChangeSummary,
    /// The [summary][`ChangeSummary`] of the changes in the [staging area].
    ///
    /// [staging area]: https://git-scm.com/docs/gitglossary#def_index
    pub staging: ChangeSummary,
    /// The number of files in the [working tree] with merge conflicts.
    ///
    /// [working tree]: https://git-scm.com/docs/gitglossary#def_working_tree
    pub conflicted: usize,
}
impl StatusSummary {
    /// Gets the [summary][`StatusSummary`] of a repository's [status].
    ///
    /// [status]: https://git-scm.com/docs/git-status
    pub fn from_repo(repo: &Repository) -> Result<Self, Error> {
        let mut working_tree = ChangeSummary::default();
        let mut staging = ChangeSummary::default();
        let mut conflicted = 0;

        for status in repo
            .statuses(Some(
                StatusOptions::new()
                    .include_untracked(true)
                    .renames_from_rewrites(false)
                    .renames_head_to_index(false)
                    .renames_index_to_workdir(false),
            ))?
            .iter()
        {
            let status = status.status();

            if status.is_empty() {
                continue;
            }

            if status.is_index_new() {
                staging.added += 1;
            } else if status.is_index_deleted() {
                staging.deleted += 1;
            } else if (Status::INDEX_MODIFIED | Status::INDEX_RENAMED | Status::INDEX_TYPECHANGE)
                .intersects(status)
            {
                staging.modified += 1;
            }

            if status.is_wt_new() {
                working_tree.added += 1;
            } else if status.is_wt_deleted() {
                working_tree.deleted += 1;
            } else if (Status::WT_MODIFIED | Status::WT_RENAMED | Status::WT_TYPECHANGE)
                .intersects(status)
            {
                working_tree.modified += 1;
            }

            if status.is_conflicted() {
                conflicted += 1;
            }
        }

        Ok(Self {
            working_tree,
            staging,
            conflicted,
        })
    }

    /// Tell if the status summary indicates the presence of changes, staged or not.
    pub fn any_changes(&self) -> bool {
        self.conflicted != 0 || self.working_tree.any_changes() || self.staging.any_changes()
    }
}

/// A summary of the changes in either a [working tree] or a [staging area].
///
/// [working tree]: https://git-scm.com/docs/gitglossary#def_working_tree
/// [staging area]: https://git-scm.com/docs/gitglossary#def_index
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChangeSummary {
    /// The number of added files.
    pub added: usize,
    /// The number of modified files.
    pub modified: usize,
    /// The number of deleted files.
    pub deleted: usize,
}
impl ChangeSummary {
    /// Tell if the summary indicates the presence of changes.
    pub fn any_changes(&self) -> bool {
        self.added != 0 || self.modified != 0 || self.deleted != 0
    }
}
