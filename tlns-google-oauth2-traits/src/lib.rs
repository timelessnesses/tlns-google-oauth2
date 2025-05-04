#![doc = include_str!("../README.md")]

use std::fmt::{Debug, Display};

/// Converting enum scopes to string
pub trait ToGoogleScope: Debug {
    /// Converting the enum back to [`str`] literal
    fn to_google_scope(&self) -> &'static str;
}

/// Converting scope strings to `T`
pub trait FromGoogleScope<T>: Debug {
    /// Converting Google Scope string to enum
    /// This might return [`Err`] if you input an invalid Google Scope.
    fn from_google_scope(google_scope: &str) -> Result<T, ()>;
}

impl Display for dyn ToGoogleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_google_scope())
    }
}