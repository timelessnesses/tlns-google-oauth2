#![doc = include_str!("../README.md")]

use std::fmt::Debug;

use oauth2::{
    self,
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenResponse, BasicTokenType,
    },
    CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, StandardRevocableToken,
};

pub use tlns_google_oauth2_traits::{FromGoogleScope, ToGoogleScope};

#[cfg(feature = "grouped-scopes")]
pub mod grouped_scopes;

#[cfg(feature = "scopes")]
pub mod scopes;

#[cfg(not(any(feature = "grouped-scopes", feature = "scopes")))]
compile_error!("You must enable either `grouped-scopes` or `scopes` feature to use this crate.");

/// A thin wrapper around [`oauth2`] for Google OAuth2.  
/// ```should_panic
/// use tlns_google_oauth2::GoogleOAuth2Client;
/// 
/// #[pollster::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = GoogleOAuth2Client::new(
///         "client_id",
///         "client_secret",
///         "redirect_uri",
///     )?;
///     let auth_url = client.build_authorize_url(
///         None,
///         &[&tlns_google_oauth2::grouped_scopes::GoogleOAuth2APIv2::AuthUserinfoProfile, &tlns_google_oauth2::scopes::Scopes::AuthUserinfoEmail],
///     );
///     println!("Redirect to {}", auth_url.redirect_url);
/// 
///     let token = client.get_token("code", None).await?;
///     println!("Token: {:?}", token);
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct GoogleOAuth2Client {
    // this is stupid
    /// OAuth2 client
    pub client: oauth2::Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
}

#[derive(Clone, Debug)]
/// Authentication stuffs
pub struct Authentication<'a> {
    /// URL to redirect user to
    pub redirect_url: String,
    /// CSRF token
    pub csrf_token: CsrfToken,
    /// Scopes that you used in [`crate::GoogleOAuth2Client::build_authorize_url`]
    pub scopes: &'a [&'a dyn ToGoogleScope],
}

impl GoogleOAuth2Client {
    /// Create new [`crate::GoogleOAuth2Client`] instance
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Result<Self, oauth2::url::ParseError> {
        let url = oauth2::RedirectUrl::new(redirect_uri.into())?;
        Ok(Self {
            client: oauth2::basic::BasicClient::new(oauth2::ClientId::new(client_id.into()))
                .set_client_secret(oauth2::ClientSecret::new(client_secret.into()))
                .set_auth_uri(
                    oauth2::AuthUrl::new(
                        "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                    )
                    .unwrap(),
                )
                .set_token_uri(
                    oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                        .unwrap(),
                )
                .set_redirect_uri(url),
        })
    }

    /// Make a authorization URL for user to authenticate
    /// `csrf_token` will be default [`oauth2::CsrfToken::new_random`]
    /// [`crate::grouped_scopes`] example will be
    /// ```rust
    /// &[&tlns_google_oauth2::grouped_scopes::GoogleOAuth2APIv2::AuthUserinfoProfile];
    /// ```
    /// Or like this!
    /// ```rust
    /// use std::str::FromStr;
    /// use tlns_google_oauth2::FromGoogleScope;
    /// &[&tlns_google_oauth2::scopes::Scopes::from_google_scope("https://www.googleapis.com/auth/userinfo.profile").unwrap()];
    /// // or
    /// &[&tlns_google_oauth2::scopes::Scopes::from_str("https://www.googleapis.com/auth/userinfo.profile").unwrap()];
    /// ```
    /// Or if you are using [`crate::scopes`]
    /// ```rust
    /// &[&tlns_google_oauth2::scopes::Scopes::AuthUserinfoProfile];
    /// ```
    pub fn build_authorize_url<'a>(
        &self,
        csrf_token: Option<fn() -> CsrfToken>,
        scopes: &'a [&'a dyn ToGoogleScope],
    ) -> Authentication<'a> {
        let auth_req = self
            .client
            .authorize_url(csrf_token.unwrap_or(CsrfToken::new_random))
            .add_scopes(
                scopes
                    .iter()
                    .map(|e| oauth2::Scope::new(e.to_google_scope().to_string())),
            );

        let res = auth_req.url();
        Authentication {
            redirect_url: res.0.to_string(),
            csrf_token: res.1,
            scopes,
        }
    }

    /// Get authentication tokens from provider with authenticated code from Google
    /// Typically you will get it from the query string of the redirected URL
    pub async fn get_token(
        &self,
        auth_code: impl Into<String>,
        http_client: Option<&oauth2::reqwest::Client>,
    ) -> Result<
        oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        oauth2::RequestTokenError<
            oauth2::HttpClientError<oauth2::reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    > {
        // i love you rust
        let http_client = http_client
            .map(|i| std::borrow::Cow::Borrowed(i))
            .unwrap_or(std::borrow::Cow::Owned(oauth2::reqwest::Client::new()));
        let res = self
            .client
            .exchange_code(oauth2::AuthorizationCode::new(auth_code.into()))
            .request_async(http_client.as_ref())
            .await?;
        Ok(res)
    }

    /// Refresh your token in case it has expired
    /// You can retrieve it from [`oauth2::StandardTokenResponse::refresh_token`]
    pub async fn refresh_token(
        &self,
        refresh_token: impl Into<String>,
        http_client: Option<&oauth2::reqwest::Client>,
    ) -> Result<
        oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        oauth2::RequestTokenError<
            oauth2::HttpClientError<oauth2::reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    > {
        // i love you rust
        let http_client = http_client
            .map(|i| std::borrow::Cow::Borrowed(i))
            .unwrap_or(std::borrow::Cow::Owned(oauth2::reqwest::Client::new()));
        let res = self
            .client
            .exchange_refresh_token(&oauth2::RefreshToken::new(refresh_token.into()))
            .request_async(http_client.as_ref())
            .await?;
        Ok(res)
    }
}
