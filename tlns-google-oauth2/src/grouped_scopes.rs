#![allow(non_camel_case_types)]
//! A bunch of scopes grouped by their headers.
//! Example
//! ```rust
//! tlns_google_oauth2::grouped_scopes::GoogleOAuth2APIv2::AuthUserinfoProfile;
//! ```

use tlns_google_oauth2_proc;
tlns_google_oauth2_proc::generate_grouped_scopes_enums!();
