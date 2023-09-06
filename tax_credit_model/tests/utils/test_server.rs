use rocket::{http::ContentType, local::asynchronous::Client};
use serde::Serialize;
use tax_credit_model_server::server::{init_service, ServerConfiguration};

#[derive(Debug)]
pub struct TestServer {
    server_client: Client,
}

pub enum Method {
    Post,
}

impl TestServer {
    pub async fn spawn(configuration: ServerConfiguration) -> TestServer {
        let client = Client::tracked(init_service(configuration));
        TestServer {
            server_client: client.await.expect("Should spawn server"),
        }
    }

    pub async fn invoke_template<Request>(
        &self,
        method: Method,
        path: &str,
        request_model: &Request,
    ) -> String
    where
        Request: Serialize,
    {
        let request = match method {
            Method::Post => self.server_client.post(path),
        }
        .header(ContentType::Form)
        .body(serde_qs::to_string(&request_model).expect("Should be a valid request"));
        let response = request.dispatch().await;

        response
            .into_string()
            .await
            .expect("Should convert body to string")
    }
}
