use rocket::local::asynchronous::Client;
use serde::{de::DeserializeOwned, Serialize};
use tax_credit_model_server::server::{init_service, ServerConfiguration};

#[derive(Debug)]
pub struct TestServer {
    server_client: Client,
}

pub enum Method {
    Post,
    Get,
    Put,
    Delete,
}

impl TestServer {
    pub async fn spawn(configuration: ServerConfiguration) -> TestServer {
        let client = Client::tracked(init_service(configuration));
        TestServer {
            server_client: client.await.expect("Should spawn server"),
        }
    }

    pub async fn invoke<Request, Response>(
        &self,
        method: Method,
        path: &str,
        request_model: &Request,
    ) -> Response
    where
        Response: DeserializeOwned,
        Request: Serialize,
    {
        let request = match method {
            Method::Post => self.server_client.post(path),
            Method::Get => self.server_client.get(path),
            Method::Put => self.server_client.put(path),
            Method::Delete => self.server_client.delete(path)
        }.body(serde_urlencoded::to_string(&request_model).expect("Should be a valid request"));

        let response = request.dispatch().await;
        let response_text = response
            .into_string()
            .await
            .expect("Should convert body to string");

        serde_urlencoded::from_str(&response_text).expect("Should be a valid response")
    }
}
