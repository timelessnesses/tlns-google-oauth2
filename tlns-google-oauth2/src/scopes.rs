#![allow(non_camel_case_types)]
//! A bunch of scopes wrapped in [`Scopes`].
//! Example:
//! ```rust
//! tlns_google_oauth2::scopes::Scopes::AuthUserinfoProfile;
//! ```

use tlns_google_oauth2_proc;
tlns_google_oauth2_proc::generate_scopes_enums!();
