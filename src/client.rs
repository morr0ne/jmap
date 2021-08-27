use indexmap::IndexMap;
use serde::de::DeserializeOwned;

use crate::types::{AnyResult, HttpClient, Request, Response, Session};

pub struct Client {
    http_client: HttpClient,
    email: Option<String>,
    password: Option<String>,
}

pub struct ClientBuilder;

impl ClientBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn build(self) -> AnyResult<Client> {
        let http_client = HttpClient::builder().build()?;
        Ok(Client {
            http_client,
            email: None,
            password: None,
        })
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
    pub async fn auth(&mut self, server: &str, email: &str, password: &str) -> AnyResult<Session> {
        let session = self
            .http_client
            .get(server)
            .basic_auth(email, Some(password))
            .send()
            .await?
            .json()
            .await?;

        self.email = Some(email.to_string());
        self.password = Some(password.to_string());

        Ok(session)
    }

    pub async fn send<T: DeserializeOwned>(
        &self,
        session: &Session,
        request: Request,
    ) -> AnyResult<T> {
        let response = self
            .http_client
            .post(session.api_url.clone())
            .basic_auth(
                &self.email.as_ref().unwrap(),
                Some(self.password.as_ref().unwrap()),
            )
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn echo(&self, session: &Session) -> AnyResult<Response> {
        let request = Request {
            using: vec![String::from("urn:ietf:params:jmap:core")],
            method_calls: vec![(
                String::from("Core/echo"),
                IndexMap::new(),
                String::from("c0"),
            )],
            created_ids: None,
        };

        let response = self.send(session, request).await?;

        Ok(response)
    }
}
