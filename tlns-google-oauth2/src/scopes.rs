#![allow(non_camel_case_types)]

use tlns_google_oauth2_proc;

#[cfg(feature = "grouped-scopes")]
tlns_google_oauth2_proc::generate_grouped_scopes_enums!();

#[cfg(feature = "scopes")]
tlns_google_oauth2_proc::generate_scopes_enums!();