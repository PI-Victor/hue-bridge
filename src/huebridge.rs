// Copyright 2022  Palade Ionut Victor
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Bridge;
use crate::Result;
use reqwest::{Client, Method, Request};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

/// HueBridge client
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct HueBridge {
    pub(crate) client: reqwest::Client,
    pub(crate) api_url: String,
    app_name: String,
    pub(crate) token: String,
}

impl HueBridge {
    /// Builds a new base client that can be used to register a new application
    /// or use an already existing one.
    /// ```
    /// let client = HueBridge::new("https://my-local-bridge.local")
    /// .with_ca_pem("./path/to/hue/bridge/root-ca")
    /// .await?
    /// ```
    pub fn new(api_url: &str) -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            api_url: api_url.to_string(),
            token: "".to_string(),
            app_name: "".to_string(),
        }
    }

    /// Set Root CA in PEM format for client validation
    pub async fn with_ca_pem<P: AsRef<Path>>(mut self, ca_path: P) -> Result<Self> {
        let cert = fs::read(ca_path).await?;
        let cert = reqwest::tls::Certificate::from_pem(&cert)?;
        self.client = Client::builder()
            .https_only(true)
            .danger_accept_invalid_hostnames(true)
            .add_root_certificate(cert)
            .build()?;

        Ok(self)
    }

    async fn check_version(&self) -> Result<()> {
        let url = format!("{}/api/0/config", &self.api_url);
        let resp = self.client.get(url).send().await?;
        let text = resp.text().await?;
        let bridge: Bridge = serde_json::from_str(&text)?;
        bridge.check_version()?;

        Ok(())
    }

    /// Registers a new application
    pub async fn register(mut self, app_name: &str) -> Result<HueBridge> {
        let _ = &self.check_version().await?;

        #[derive(Debug, Clone, Deserialize)]
        struct HueResult {
            success: Option<Success>,
            error: Option<Error>,
        }

        #[derive(Debug, Clone, Deserialize)]
        struct Success {
            username: String,
        }

        #[derive(Debug, Clone, Deserialize)]
        struct Error {
            description: String,
        }

        let url = format!("{}/api", &self.api_url);

        #[derive(Serialize, Deserialize)]
        struct Application {
            devicetype: String,
            generateclientkey: bool,
        }

        let app = Application {
            devicetype: app_name.to_string(),
            generateclientkey: true,
        };

        let req_body = serde_json::to_string(&app)?;
        let resp = self.client.post(url).body(req_body).send().await?;

        let resp_body = resp.text().await?;
        let success: Vec<HueResult> = serde_json::from_str(&resp_body)?;
        if let Some(e) = &success[0].error {
            Err(format!("{}", &e.description).into())
        } else {
            let res = success.get(0).unwrap().clone();
            self.token = res.success.unwrap().username;
            self.app_name = app_name.to_string();

            Ok(self)
        }
    }

    /// Instantiates a hue bridge client with a registered username
    pub async fn with_username(mut self, username: &str) -> Result<Self> {
        let _ = &self.check_version().await?;

        self.token = username.to_string();
        Ok(self)
    }

    /// Retrieve the username after an application has been registered. Can be used in conjuction with
    /// HueBridge::with_username()
    pub fn get_username(&self) -> String {
        self.token.clone()
    }

    /// Retrieve the registered application name
    pub fn get_app_name(&self) -> String {
        // TODO: retrieve this from the user config!
        self.app_name.clone()
    }

    pub(crate) async fn fetch_resource<T>(&self, method: Method, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", &self.api_url, endpoint).parse::<reqwest::Url>()?;
        let mut req = Request::new(method, url);

        req.headers_mut()
            .append("hue-application-key", self.token.parse()?);

        let resp = self.client.execute(req).await?;
        let body = &resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }
}

impl Default for HueBridge {
    fn default() -> Self {
        let client = Client::new();

        Self {
            client,
            api_url: "".to_string(),
            token: "".to_string(),
            app_name: "".to_string(),
        }
    }
}
