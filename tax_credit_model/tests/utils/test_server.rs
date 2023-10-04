use std::collections::HashMap;

use rocket::{http::ContentType, local::asynchronous::Client};
use serde::Serialize;
use tax_credit_model_server::server::{init_service, Dependencies, ServerConfiguration};

#[derive(Debug)]
pub struct TestServer {
    server_client: Client,
}

pub enum Method {
    Post,
}

pub struct Response<T> {
    pub headers: HashMap<String, String>,
    pub data: T,
}

impl TestServer {
    pub async fn spawn(
        configuration: ServerConfiguration,
        dependencies: Dependencies,
    ) -> TestServer {
        let client = Client::tracked(init_service(configuration, dependencies));
        TestServer {
            server_client: client.await.expect("Should spawn server"),
        }
    }

    pub async fn invoke_template<'a, Request>(
        &self,
        method: Method,
        path: &str,
        request_model: &Request,
    ) -> Response<String>
    where
        Request: Serialize,
    {
        let request = match method {
            Method::Post => self.server_client.post(path),
        }
        .header(ContentType::Form)
        .body(serde_qs::to_string(&request_model).expect("Should be a valid request"));
        let response = request.dispatch().await;
        let mut headers = HashMap::new();
        for header in response.headers().iter() {
            headers.insert(header.name.to_string(), header.value().to_string());
        }

        Response {
            headers,
            data: response
                .into_string()
                .await
                .expect("Should convert body to string"),
        }
    }
}
