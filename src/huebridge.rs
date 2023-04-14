// Copyright 2022  Palade Ionut Victor
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy
// of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

use crate::{Bridge, Result};
use reqwest::tls::Certificate;
use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

/// HueBridge client
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct HueBridge {
    client: Option<Client>,
    pub(crate) api_url: Option<String>,
    app_name: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) ca_path: Option<PathBuf>,
    pub(crate) disable_tls: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct HueResult {
    success: Option<Success>,
    error: Option<HueError>,
}

#[derive(Debug, Clone, Deserialize)]
struct Success {
    username: String,
}

#[derive(Debug, Clone, Deserialize)]
struct HueError {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Application {
    devicetype: String,
    generateclientkey: bool,
}

impl HueBridge {
    /// Builds a new base client that can be used to register a new
    /// application or use an already existing one.
    /// ```
    /// let client = HueBridge::builder()
    ///     .api("https://my-local-bridge.local")
    ///     .ca_pem("./path/to/hue/bridge/root-ca")
    ///     .token("my-token")
    ///     .app_name("my-app")
    ///     .build()
    ///     .await?;
    /// ```
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set Root CA in PEM format for client validation
    pub fn ca_pem(mut self, ca_path: impl AsRef<Path>) -> Self {
        self.ca_path = Some(ca_path.as_ref().to_path_buf());
        self
    }

    /// Instantiates a hue bridge client with a registered username
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }
    /// Set the API URL for the hue bridge client
    pub fn api(mut self, api_url: &str) -> Self {
        self.api_url = Some(api_url.to_string());
        self
    }

    /// Disable TLS validation for the hue bridge client
    pub fn disable_tls(mut self) -> Self {
        self.disable_tls = true;
        self
    }
    /// Registers or finds a new application on the hue bridge
    pub fn app_name(mut self, app_name: &str) -> Self {
        self.app_name = Some(app_name.to_string());
        self
    }

    /// Register or finds a new application with the hue bridge
    pub async fn build(mut self) -> Result<HueBridge> {
        let cert_file = fs::read(self.ca_path.as_ref().unwrap()).await?;
        let cert = Certificate::from_pem(&cert_file)?;
        self.client = Some(
            Client::builder()
                .https_only(true)
                .danger_accept_invalid_hostnames(true)
                .add_root_certificate(cert)
                .build()?,
        );

        if let Err(e) = &self.check_version().await {
            return Err(format!("failed to check version: {e}").into());
        };
        if self.token.is_none() {
            self.register().await
        } else {
            Ok(HueBridge { ..self })
        }
    }

    async fn register(mut self) -> Result<Self> {
        let app_name = &self.app_name.as_deref().to_owned().unwrap();

        let app = Application {
            devicetype: app_name.to_string(),
            generateclientkey: true,
        };

        let req_body = serde_json::to_string(&app)?;
        let resp = self
            .do_request::<Vec<HueResult>>(Method::POST, "api", Some(req_body))
            .await?;

        if let Some(err) = &resp[0].error {
            Err(err.description.clone().into())
        } else {
            let res = resp.get(0).unwrap().clone();
            self.token = Some(res.success.unwrap().username);
            self.app_name = Some(app_name.to_string());
            Ok(HueBridge { ..self.clone() })
        }
    }

    async fn check_version(&self) -> Result<()> {
        let resp: Bridge = self.do_request(Method::GET, "api/config", None).await?;
        resp.check_version()?;
        Ok(())
    }

    /// Retrieve the username after an application has been registered. Can be
    /// used in conjuction with HueBridge::with_username()
    pub fn get_username(&self) -> &str {
        self.token.as_ref().unwrap()
    }

    /// Retrieve the registered application name
    pub fn get_app_name(&self) -> &str {
        // TODO: retrieve this from the user config!
        self.app_name.as_ref().unwrap()
    }

    pub(crate) async fn do_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        data: Option<String>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = Url::parse(format!("{}/{}", &self.api_url.as_ref().unwrap(), endpoint).as_str())?;
        let client = self.client.as_ref().unwrap().to_owned();
        let mut req = if let Some(data) = data {
            client.request(method, url).body(data)
        } else {
            client.request(method, url)
        };
        let token = self.token.as_ref().unwrap().to_string();
        req = req.header("hue-application-key", token.clone());
        let resp = req.send().await?;
        let body = resp.text().await?;

        Ok(serde_json::from_str(&body)?)
    }
}

impl Default for HueBridge {
    fn default() -> Self {
        Self {
            client: None,
            disable_tls: false,
            api_url: None,
            app_name: None,
            token: None,
            ca_path: None,
        }
    }
}
