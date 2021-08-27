use crate::types::{AnyResult, HttpClient, Session};

pub struct Client {
    http_client: HttpClient,
}

pub struct ClientBuilder;

impl ClientBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn build(self) -> AnyResult<Client> {
        let http_client = HttpClient::builder().build()?;
        Ok(Client { http_client })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    // TODO: better error handling
    pub async fn auth(&self, server: &str, email: &str, password: &str) -> AnyResult<Session> {
        let session = self
            .http_client
            .get(server)
            .basic_auth(email, Some(password))
            .send()
            .await?
            .json()
            .await?;

        Ok(session)
    }

    pub async fn send(&self, session: &Session) -> AnyResult<()> {
        Ok(())
    }
}
