use crate::models::auth::{Auth, AuthFromSession, AuthFromSessionError};
use crate::models::stats::StatsResponse;
use crate::models::user::User;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::Deserialize;

pub mod models;

/// The error type
type Error = Box<dyn std::error::Error>;

/// The user agent used for requests.
pub const USER_AGENT: &str = "xfbridge";

/// The XenForo bridge api.
#[derive(Debug, Clone)]
pub struct XfBridge {
    base_url: String,
    api_key: String,
    su_id: Option<i32>
}

impl XfBridge {
    /// Creates a XenForo bridge with a specified api key and base url.
    ///
    /// # Arguments
    /// * `base_url`    - The base url to access.
    /// * `api_key`     - The api key to use.
    /// * `su_id`       - The id of the user if the targeted endpoint requires a super user key.
    pub fn new(base_url: &str, api_key: &str, &su_id: Option<i32>) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            su_id
        }
    }

    /// Builds a client with the required headers.
    fn build_client(&self) -> Result<Client, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_str(USER_AGENT)?);
        headers.insert("XF-Api-Key", HeaderValue::from_str(&self.api_key)?);
        if &self.su_id.is_some() {
            headers.insert("XF-Api-User", HeaderValue::from(&self.su_id.unwrap()));
        }
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/x-www-form-urlencoded")?,
        );

        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|e| e.into())
    }

    /// Gets the forum stats.
    pub async fn get_stats(&self) -> Result<StatsResponse, Error> {
        let client = self.build_client()?;
        let response = client
            .get(format!("{}/api/stats", &self.base_url))
            .send()
            .await?;

        response.json().await.map_err(|e| e.into())
    }

    /// Logs in to XenForo with a specified username and password.
    ///
    /// # Arguments
    /// * `username`    - The username.
    /// * `password`    - The password.
    pub async fn login_with_name_and_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, Error> {
        let client = self.build_client()?;
        let auth = Auth {
            login: username.into(),
            password: password.into(),
        };

        let response = client
            .post(format!("{}/api/auth", &self.base_url))
            .body(serde_urlencoded::to_string(&auth)?)
            .send()
            .await?;

        response.json().await.map_err(|e| e.into())
    }

    /// Logs in to XenForo with a session token.
    ///
    /// # Arguments
    /// * `session` - The session token.
    pub async fn login_with_session(&self, session: &str) -> Result<User, Error> {
        let client = self.build_client()?;
        let auth = AuthFromSession {
            session_id: session.into(),
        };

        let response = client
            .post(format!("{}/api/auth/from-session", &self.base_url))
            .body(serde_urlencoded::to_string(&auth)?)
            .send()
            .await?;

        #[derive(Deserialize)]
        pub struct AuthFromSessionResponse {
            user: Option<User>,
        }

        let response = response.json::<AuthFromSessionResponse>().await?;
        if let Some(user) = response.user {
            return Ok(user);
        }

        Err(AuthFromSessionError::InvalidSession.into())
    }
}
