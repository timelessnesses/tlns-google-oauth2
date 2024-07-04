#![doc = include_str!("../README.md")]

/// Converting enum scopes to string
pub trait ToGoogleScope {
    /// Converting the enum back to [`&'static str`]
    fn to_google_scope(&self) -> &'static str;
}

/// Converting scope strings to [`T`]
pub trait FromGoogleScope<T> {
    /// Converting Google Scope to enum
    /// This might return [`Err`] if you input an invalid Google Scope.
    fn from_google_scope(google_scope: &str) -> Result<T, ()>;
}
