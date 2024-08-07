#![doc = include_str!("../README.md")]

use core::panic::{RefUnwindSafe, UnwindSafe};

use oauth2::{self, basic::BasicTokenType, CsrfToken, EmptyExtraTokenFields};

pub mod grouped_scopes;
pub mod scopes;
pub use tlns_google_oauth2_traits::{FromGoogleScope, ToGoogleScope};

/// A thin wrapper around [`oauth2`] for Google OAuth2.
pub struct GoogleOAuth2Client<'b> {
    client: oauth2::basic::BasicClient,
    redirect_uri: std::borrow::Cow<'b, oauth2::RedirectUrl>,
}

/// Scope types ([`tlns_google_oauth2_traits::ToGoogleScope`])
pub type Scope = dyn ToGoogleScope + Send + Sync + UnwindSafe + RefUnwindSafe;

/// Authentication stuffs
pub struct Authentication<'a>(pub String, pub CsrfToken, pub Vec<&'a Scope>);

impl<'b> GoogleOAuth2Client<'b> {
    /// Create new [`crate::GoogleOAuth2Client`] instance
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Result<Self, oauth2::url::ParseError> {
        let url = oauth2::RedirectUrl::new(redirect_uri)?;
        Ok(Self {
            client: oauth2::basic::BasicClient::new(
                oauth2::ClientId::new(client_id),
                Some(oauth2::ClientSecret::new(client_secret)),
                oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                    .unwrap(),
                Some(
                    oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                        .unwrap(),
                ),
            ),
            redirect_uri: std::borrow::Cow::Owned(url),
        })
    }

    /// Make a authorization URL for user to authenticate
    /// `csrf_token` will be default [`oauth2::CsrfToken::new_random`]
    /// [`crate::grouped_scopes`] example will be
    /// ```rust
    /// vec![&tlns_google_oauth2::grouped_scopes::GoogleOAuth2APIv2::AuthUserinfoProfile];
    /// ```
    /// Or like this!
    /// ```rust
    /// use tlns_google_oauth2::FromGoogleScope;
    /// vec![&tlns_google_oauth2::scopes::Scopes::from_google_scope("https://www.googleapis.com/auth/userinfo.profile").unwrap()];
    /// ```
    /// Or if you are using [`crate::scopes`]
    /// ```rust
    /// vec![&tlns_google_oauth2::scopes::Scopes::AuthUserinfoProfile];
    /// ```
    pub fn authorize_url<'a>(
        &self,
        csrf_token: Option<fn() -> CsrfToken>,
        scopes: Vec<&'a Scope>,
    ) -> Result<Authentication<'a>, String> {
        let auth_req = self
            .client
            .authorize_url(csrf_token.unwrap_or(CsrfToken::new_random))
            .add_scopes(
                scopes
                    .iter()
                    .map(|e| oauth2::Scope::new(e.to_google_scope().to_string())),
            )
            .set_redirect_uri(self.redirect_uri.clone());

        let res = auth_req.url();
        Ok(Authentication(res.0.to_string(), res.1, scopes))
    }

    /// Get authentication tokens from provider with authenticated code from Google
    pub async fn get_token(
        &self,
        auth_code: String,
    ) -> Result<oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, String> {
        let res = self
            .client
            .exchange_code(oauth2::AuthorizationCode::new(auth_code))
            .set_redirect_uri(self.redirect_uri.clone())
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| e.to_string())?;
        Ok(res)
    }
}
