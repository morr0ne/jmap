use serde::de::DeserializeOwned;

use crate::{
    client::Auth,
    types::{
        AnyResult, HttpClient, Invocation, JsonValue, Request, RequestBuilder, Response,
        SessionObject,
    },
};

pub struct Session {
    pub(crate) http_client: HttpClient,
    pub(crate) auth: Auth,
    pub(crate) session_object: SessionObject,
}

impl Session {
    pub async fn send<T: DeserializeOwned>(&self, request: Request) -> AnyResult<T> {
        let req = self.http_client.post(self.session_object.api_url.clone());

        let response = match self.auth {
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

    pub async fn echo(&self) -> AnyResult<Response> {
        let request = RequestBuilder::new()
            .capability("urn:ietf:params:jmap:core")
            .method(Invocation::new("Core/echo", [], "c0"))
            .build();

        let response = self.send(request).await?;

        Ok(response)
    }

    pub async fn mailbox(&self) -> AnyResult<Response> {
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

        let response = self.send(request).await?;

        Ok(response)
    }
}
