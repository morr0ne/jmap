use crate::{
    session::Session,
    types::{AnyResult, HttpClient, SessionObject},
};

pub enum Auth {
    Basic(String, Option<String>),
    Bearer(String),
}

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
    pub async fn auth(&mut self, server: &str, auth: Auth) -> AnyResult<Session> {
        let req = self.http_client.get(server);

        let session_object = match auth {
            Auth::Basic(ref username, ref password) => req.basic_auth(username, password.as_ref()),
            Auth::Bearer(ref token) => req.bearer_auth(token),
        }
        .send()
        .await?
        .json::<SessionObject>()
        .await?;

        Ok(Session {
            http_client: self.http_client.clone(),
            auth,
            session_object,
        })
    }
}
