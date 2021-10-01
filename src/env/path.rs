//! [`Path`] utilities.

#[cfg(test)]
mod test;

use dirs;
use std::{
    env, error, fmt, io,
    ops::Deref,
    path::{Component, Path, PathBuf},
};

/// Finds the first [ancestor](`Path::ancestors`) of `path` matching `base`, if any.
///
/// The comparisons between the ancestors of `path` and `base`
/// take some normalization into account.
/// It is the same normalization done by [`Path::components`],
/// but occurrences of `.` are always normalized away,
/// even if they are at the beginning of the path.
///
/// A relative path never matches an absolute path and vice versa, so,
/// if `base` and `path` are not either both relative or both absolute, [`None`] is returned.
///
/// [Windows path prefixes](`Component::Prefix`) are considered when comparing the paths,
/// so, if either `base` or `path` have a prefix, then both must have the same prefix,
/// otherwise [`None`] is returned.
pub fn find_ancestor<'b, 'p>(base: &'b Path, path: &'p Path) -> Option<&'p Path> {
    let mut base_components = base.components();
    let mut path_components = path.components();

    let ancestors = {
        let mut ancestors = path.ancestors().collect::<Vec<_>>();
        ancestors.reverse();
        ancestors
    };

    loop {
        match base_components.next() {
            Some(Component::Prefix(base_prefix)) => match path_components.next() {
                Some(Component::Prefix(path_prefix)) if base_prefix == path_prefix => continue,
                _ => break None,
            },
            Some(Component::RootDir) => match path_components.next() {
                Some(Component::RootDir) => continue,
                _ => break None,
            },
            Some(Component::CurDir) => continue,
            Some(base_component) => {
                let path_component = match path_components.next() {
                    Some(Component::CurDir) => match path_components.next() {
                        Some(path_component) => path_component,
                        _ => break None,
                    },
                    Some(path_component @ (Component::ParentDir | Component::Normal(_))) => {
                        path_component
                    }
                    _ => break None,
                };
                if base_component != path_component {
                    break None;
                }

                let mut ancestor_index = 1;
                break loop {
                    match base_components.next() {
                        None => break Some(ancestors[ancestor_index]),
                        Some(base_component) => match path_components.next() {
                            Some(path_component) => {
                                if path_component == base_component {
                                    ancestor_index += 1;
                                    continue;
                                } else {
                                    break None;
                                }
                            }
                            None => break None,
                        },
                    }
                };
            }
            None => match path_components.next() {
                None | Some(Component::CurDir | Component::ParentDir | Component::Normal(_)) => {
                    break Some(ancestors[0]);
                }
                _ => break None,
            },
        }
    }
}

/// Strips the first [ancestor](`Path::ancestors`) of `path`
/// that matches `base` as defined by [`find_ancestor`].
///
/// The result is `path` represented as relative to `base`.
///
/// # Errors
///
/// When `base` is not an ancestor of `path` as defined by [`find_ancestor`],
/// returns [`Err`] with [`StripAncestorError::BaseNotAnAcestorError`].
pub fn strip_ancestor<'b, 'p>(
    base: &'b Path,
    path: &'p Path,
) -> Result<&'p Path, StripAncestorError> {
    let ancestor = find_ancestor(base, path).ok_or(StripAncestorError::BaseNotAnAcestorError)?;

    // `.strip_prefix` only fails when `ancestor` is not a prefix of `path`,
    // which is impossible as `ancestor` was taken from `path.ancestors()`.
    path.strip_prefix(ancestor)
        .map_err(|_| StripAncestorError::BaseNotAnAcestorError)
}

/// Error of [`strip_ancestor`].
#[derive(Debug)]
pub enum StripAncestorError {
    BaseNotAnAcestorError,
}
impl error::Error for StripAncestorError {}
impl fmt::Display for StripAncestorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StripAncestorError::BaseNotAnAcestorError => {
                write!(f, "`base` was not an ancestor of `path`.")
            }
        }
    }
}

/// Abbreviates `path` by replacing its `base` acestor with `abbreviation`.
///
/// # Errors
///
/// When [striping](`strip_ancestor`) the `base` ancestor from `path` fails,
/// the error is bubbled up.
pub fn abbreviate_path(
    base: &Path,
    abbreviation: &Path,
    path: &Path,
) -> Result<PathBuf, StripAncestorError> {
    strip_ancestor(base, path).map(|relative_path| abbreviation.join(relative_path))
}

/// [Abbreviates](`abbreviate_path`) `path`
/// by replacing its [home](`dirs::home_dir()`) acestor with `~`.
///
/// Returns an [`AbbreviateHomeResult`] holding either the abbreviated path
/// ([`Abbreviated`](`AbbreviateHomeResult::Abbreviated`) variant) or,
/// when the [home dir](`dirs::home_dir`) is not found or is not an ancestor of the path,
/// the path unchanged ([`NoHome`](`AbbreviateHomeResult::NoHome`) and
/// [`HomeNotAnAcestor`](`AbbreviateHomeResult::HomeNotAnAcestor`) variants, respectively).
pub fn abbreviate_home<P: Deref<Target = Path>>(path: P) -> AbbreviateHomeResult<P> {
    match dirs::home_dir() {
        Some(home_dir) => match abbreviate_path(&*home_dir, "~".as_ref(), &*path) {
            Ok(abbreviated) => AbbreviateHomeResult::Abbreviated(abbreviated),
            Err(StripAncestorError::BaseNotAnAcestorError) => {
                AbbreviateHomeResult::HomeNotAnAcestor { path }
            }
        },
        None => AbbreviateHomeResult::NoHome { path },
    }
}

/// [`Ok`] variant of [`abbreviate_home`]'s return.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbbreviateHomeResult<P: Deref<Target = Path>> {
    /// The abbreviated path
    Abbreviated(PathBuf),
    /// Path unchanged due to the [home dir](`dirs::home_dir`)
    /// not being one of its ancestors as defined by [`find_ancestor`]
    HomeNotAnAcestor {
        /// The path, unchanged
        path: P,
    },
    /// Path unchanged due to the [home dir](`dirs::home_dir`) not being found
    NoHome {
        /// The path, unchanged
        path: P,
    },
}

/// Gets the [`current_dir`](`std::env::current_dir()`) with
/// [the home dir abbreviated](`abbreviate_home`).
///
/// # Errors
///
/// When [`std::env::current_dir`] fails, the error is bubbled up.
pub fn current_dir_abbreviated_home() -> io::Result<AbbreviateHomeResult<PathBuf>> {
    Ok(abbreviate_home(env::current_dir()?))
}
