#[cfg(test)]
mod find_ancestor {

    use crate::env::path::find_ancestor;

    macro_rules! multi_assert_find_ancestor {
        (@expected None) => {
            None
        };
        (@expected ($path:expr, $base:expr $(,)?)) => {
            find_ancestor($path.as_ref(), $base.as_ref())
        };
        (@expected $expected:expr) => {
            Some($expected.as_ref())
        };
        (@expected $expected:tt) => { $expected };
        ($(($path:expr, $base:expr $(,)?) => $expected:tt),*$(,)?) => {
            $(
                assert_eq!(
                    find_ancestor($path.as_ref(), $base.as_ref()),
                    multi_assert_find_ancestor!(@expected $expected),
                );
            )*
        };
    }

    #[test]
    fn relative_does_not_match_absolute() {
        multi_assert_find_ancestor!(
            ("/path/to", "path/to/something") => None,
            ("path/to", "/path/to/something") => None,
            ("", "/path/to/something") => None,
            ("/", "path/to/something") => None,
        );
        #[cfg(windows)]
        multi_assert_find_ancestor!(
            ("C:/path/to", "C:path/to/something") => None,
            ("C:path/to", "C:/path/to/something") => None,
            ("C:", "C:/path/to/something") => None,
            ("C:/", "C:path/to/something") => None,
        );
    }

    #[test]
    fn empty_base_always_matches_relative() {
        multi_assert_find_ancestor!(
            ("", "path/to/something") => "",
            ("", "") => "",
        );
        #[cfg(windows)]
        multi_assert_find_ancestor!(
            ("C:", "C:path/to/something") => "C:",
            ("C:", "C:") => "C:",
        );
    }

    #[test]
    fn root_base_always_matches_absolute() {
        multi_assert_find_ancestor!(
            ("/", "/path/to/something") => "/",
            ("/", "/") => "/",
        );
        #[cfg(windows)]
        multi_assert_find_ancestor!(
            ("C:/", "C:/path/to/something") => "C:/",
            ("C:/", "C:/") => "C:/",
        );
    }

    #[cfg(windows)]
    #[test]
    fn prefixes_must_match() {
        multi_assert_find_ancestor!(
            ("C:/", "D:/path/to/something") => None,
            ("C:/", "D:/") => None,
            ("C:", "D:path/to/something") => None,
            ("C:", "D:") => None,
            ("C:/path/to", "D:/path/to/something") => None,
            ("C:path/to", "D:path/to/something") => None,
        );
    }

    #[test]
    fn leading_current_dir_is_ignored() {
        multi_assert_find_ancestor!(
            ("/./path/to", "path/to/something") => ("/path/to", "path/to/something"),
            ("/path/to", "./path/to/something") => ("/path/to", "path/to/something"),
            ("/./path/to", "./path/to/something") => ("/path/to", "path/to/something"),
            ("./path/to", "/path/to/something") => ("path/to", "/path/to/something"),
            ("path/to", "/./path/to/something") => ("path/to", "/path/to/something"),
            ("./path/to", "/./path/to/something") => ("path/to", "/path/to/something"),
            (".", "/path/to/something") => ("", "/path/to/something"),
            ("", "/./path/to/something") => ("", "/path/to/something"),
            (".", "/./path/to/something") => ("", "/path/to/something"),
            ("/.", "path/to/something") => ("/", "path/to/something"),
            ("/", "./path/to/something") => ("/", "path/to/something"),
            ("/.", "./path/to/something") => ("/", "path/to/something"),
        );
        #[cfg(windows)]
        multi_assert_find_ancestor!(
            ("C:/./path/to", "C:path/to/something") => ("C:/path/to", "C:path/to/something"),
            ("C:/path/to", "C:./path/to/something") => ("C:/path/to", "C:path/to/something"),
            ("C:/./path/to", "C:./path/to/something") => ("C:/path/to", "C:path/to/something"),
            ("C:./path/to", "C:/path/to/something") => ("C:path/to", "C:/path/to/something"),
            ("C:path/to", "C:/./path/to/something") => ("C:path/to", "C:/path/to/something"),
            ("C:./path/to", "C:/./path/to/something") => ("C:path/to", "C:/path/to/something"),
            ("C:.", "C:/path/to/something") => ("C:", "C:/path/to/something"),
            ("C:", "C:/./path/to/something") => ("C:", "C:/path/to/something"),
            ("C:.", "C:/./path/to/something") => ("C:", "C:/path/to/something"),
            ("C:/.", "C:path/to/something") => ("C:/", "C:path/to/something"),
            ("C:/", "C:./path/to/something") => ("C:/", "C:path/to/something"),
            ("C:/.", "C:./path/to/something") => ("C:/", "C:path/to/something"),
        );
    }
}
