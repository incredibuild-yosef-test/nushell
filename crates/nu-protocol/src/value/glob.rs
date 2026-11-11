use serde::Deserialize;
use std::fmt::Display;

/// A glob pattern together with whether it should be expanded.
///
/// Used instead of `Value::Glob` directly, so glob handling does not have to consider
/// every variant of the `Value` enum.
#[derive(Debug, Clone, Deserialize)]
pub enum NuGlob {
    /// Don't expand the glob pattern, normally it includes a quoted string(except backtick)
    /// And a variable that doesn't annotated with `glob` type
    DoNotExpand(String),
    /// A glob pattern that is required to expand, it includes bare word
    /// And a variable which is annotated with `glob` type
    Expand(String),
}

impl NuGlob {
    /// Transforms the pattern text, keeping whether it expands.
    pub fn map(self, f: impl FnOnce(String) -> String) -> Self {
        match self {
            NuGlob::DoNotExpand(s) => NuGlob::DoNotExpand(f(s)),
            NuGlob::Expand(s) => NuGlob::Expand(f(s)),
        }
    }

    pub fn strip_ansi_string_unlikely(self) -> Self {
        self.map(nu_utils::strip_ansi_string_unlikely)
    }

    /// `true` for a pattern that must be expanded (a bare word or a `glob`-typed value).
    #[inline]
    pub fn is_expand(&self) -> bool {
        matches!(self, NuGlob::Expand(..))
    }
}

impl AsRef<str> for NuGlob {
    fn as_ref(&self) -> &str {
        match self {
            NuGlob::DoNotExpand(s) | NuGlob::Expand(s) => s,
        }
    }
}

impl Display for NuGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
