#![doc = include_str!("../README.md")]

use oauth2::{self, basic::{BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType}, CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, StandardRevocableToken};

pub mod grouped_scopes;
pub mod scopes;
pub use tlns_google_oauth2_traits::{FromGoogleScope, ToGoogleScope};

/// A thin wrapper around [`oauth2`] for Google OAuth2.
pub struct GoogleOAuth2Client {
    client: oauth2::Client<BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse, EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
}

/// Authentication stuffs
pub struct Authentication<'a>(pub String, pub CsrfToken, pub Vec<&'a dyn ToGoogleScope>);

impl GoogleOAuth2Client {
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
            ).set_client_secret(oauth2::ClientSecret::new(client_secret))
            .set_auth_uri(oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .unwrap())
            .set_token_uri(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .unwrap())
            .set_redirect_uri(url),
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
        scopes: Vec<&'a dyn ToGoogleScope>,
    ) -> Result<Authentication<'a>, String> {
        let auth_req = self
            .client
            .authorize_url(csrf_token.unwrap_or(CsrfToken::new_random))
            .add_scopes(
                scopes
                    .iter()
                    .map(|e| oauth2::Scope::new(e.to_google_scope().to_string())),
            );

        let res = auth_req.url();
        Ok(Authentication(res.0.to_string(), res.1, scopes))
    }

    /// Get authentication tokens from provider with authenticated code from Google
    pub async fn get_token(
        &self,
        auth_code: String,
        http_client: Option<oauth2::reqwest::Client>,
    ) -> Result<oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, String> {
        let http_client = http_client.unwrap_or(oauth2::reqwest::Client::new());
        let res = self
            .client
            .exchange_code(oauth2::AuthorizationCode::new(auth_code))
            .request_async(&http_client)
            .await
            .map_err(|e| e.to_string())?;
        Ok(res)
    }
}
