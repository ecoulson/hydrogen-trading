use std::fs;

use serde::Deserialize;
use tax_credit_model_server::server::{Dependencies, ServerConfiguration};

use super::test_server::TestServer;

#[derive(Deserialize, Debug)]
pub struct TestEnv {
    pub assets_directory: String,
}

impl TestEnv {
    pub fn load() -> TestEnv {
        let mut path = std::env::current_exe().expect("Should be run from a test executable");
        while !path.ends_with("target") {
            path.pop();
        }
        path.pop();
        path.push("test_env.toml");
        let content = fs::read_to_string(path).expect("Should read test_env file to string");

        toml::from_str(&content).expect("Should be valid config")
    }

    pub async fn create_test_server(&self, dependencies: Dependencies) -> TestServer {
        let configuration = ServerConfiguration::new("", &self.assets_directory);
        TestServer::spawn(configuration, dependencies).await
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {}
}
