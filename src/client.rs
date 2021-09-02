use serde::de::DeserializeOwned;

use crate::types::{
    AnyResult, HttpClient, Invocation, JsonValue, Request, RequestBuilder, Response, Session,
};

pub enum Auth {
    Basic(String, Option<String>),
    Bearer(String),
}

pub struct Client {
    http_client: HttpClient,
    auth: Option<Auth>,
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
            auth: None,
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
    pub async fn auth(&mut self, server: &str, auth: Auth) -> AnyResult<Session> {
        let req = self.http_client.get(server);

        let session = match auth {
            Auth::Basic(ref username, ref password) => req.basic_auth(username, password.as_ref()),
            Auth::Bearer(ref token) => req.bearer_auth(token),
        }
        .send()
        .await?
        .json()
        .await?;

        self.auth = Some(auth);

        Ok(session)
    }

    pub async fn send<T: DeserializeOwned>(
        &self,
        session: &Session,
        request: Request,
    ) -> AnyResult<T> {
        let req = self.http_client.post(session.api_url.clone());

        let response = match self.auth.as_ref().unwrap() {
            Auth::Basic(ref username, ref password) => req.basic_auth(username, password.as_ref()),
            Auth::Bearer(ref token) => req.bearer_auth(token),
        }
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

        Ok(response)
    }

    pub async fn echo(&self, session: &Session) -> AnyResult<Response> {
        let request = RequestBuilder::new()
            .capability("urn:ietf:params:jmap:core")
            .method(Invocation::new("Core/echo", [], "c0"))
            .build();

        let response = self.send(session, request).await?;

        Ok(response)
    }

    pub async fn mailbox(&self, session: &Session) -> AnyResult<Response> {
        let request = RequestBuilder::new()
            .capability("urn:ietf:params:jmap:core")
            .capability("urn:ietf:params:jmap:mail")
            .method(Invocation::new(
                "Mailbox/get",
                [
                    (String::from("accountId"), "ue1e14034".into()),
                    (String::from("ids"), JsonValue::Null),
                    (String::from("properties"), JsonValue::Null),
                ],
                "c0",
            ))
            .build();

        let response = self.send(session, request).await?;

        Ok(response)
    }
}
