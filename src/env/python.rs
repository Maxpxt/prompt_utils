use std::{
    env,
    ffi::{OsStr, OsString},
    ops,
    path::Path,
};

/// Gets the name of the currently active [venv], if any.
/// from an arbitrary set of environment variables.
///
/// Calling `get_env_var` with an environment variable name
/// must return that variable's value, or [`None`] if it does not exist.
///
/// [venv]: https://docs.python.org/3/library/venv.html
pub fn query_venv<T, S>(get_env_var: impl Fn(&str) -> Option<T>) -> Option<OsString>
where
    T: ops::Deref<Target = S>,
    S: AsRef<OsStr> + ?Sized,
{
    Some(
        Path::new(&*get_env_var("VIRTUAL_ENV")?)
            .file_name()?
            .to_owned(),
    )
}

/// Gets the name of the currently active [venv], if any,
/// from the [environment variables of the current process](`std::env::var_os`).
///
/// [venv]: https://docs.python.org/3/library/venv.html
pub fn query_venv_from_env() -> Option<OsString> {
    query_venv(|key| env::var_os(key))
}

/// Gets the name of the currently active [conda] environment, if any,
/// from an arbitrary set of environment variables.
///
/// Calling `get_env_var` with an environment variable name
/// must return that variable's value, or [`None`] if it does not exist.
///
/// [conda]: https://conda.io
pub fn query_conda_env<T>(get_env_var: impl Fn(&str) -> Option<T>) -> Option<T> {
    get_env_var("CONDA_DEFAULT_ENV")
}

/// Gets the name of the currently active [conda] environment, if any,
/// from the [environment variables of the current process](`std::env::var_os`).
///
/// [conda]: https://conda.io
pub fn query_conda_env_from_env() -> Option<OsString> {
    query_conda_env(|key| env::var_os(key))
}
