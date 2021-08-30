use indexmap::{indexmap, IndexMap};
use serde::{de::DeserializeOwned, Serialize};

use crate::types::{
    methods::get, AnyResult, HttpClient, Invocation, JsonValue, Request, Response, Session,
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

    pub async fn send<R: Serialize, T: DeserializeOwned>(
        &self,
        session: &Session,
        request: Request<R>,
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

    // pub async fn echo(&self, session: &Session) -> AnyResult<Response> {
    //     let request = Request {
    //         using: vec![String::from("urn:ietf:params:jmap:core")],
    //         method_calls: vec![Invocation(
    //             String::from("Core/echo"),
    //             IndexMap::new(),
    //             String::from("c0"),
    //         )],
    //         created_ids: None,
    //     };

    //     let response = self.send(session, request).await?;

    //     Ok(response)
    // }

    pub async fn mailbox(&self, session: &Session) -> AnyResult<Response> {
        let request: Request<get::Request> = Request {
            using: vec![
                String::from("urn:ietf:params:jmap:core"),
                String::from("urn:ietf:params:jmap:mail"),
            ],
            method_calls: vec![Invocation(
                String::from("Mailbox/get"),
                get::Request {
                    account_id: String::from("ue1e14034"),
                    ids: None,
                    properties: None,
                },
                String::from("c0"),
            )],
            created_ids: None,
        };

        let response = self.send(session, request).await?;

        Ok(response)
    }
}
