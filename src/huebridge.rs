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

use crate::Result;
use reqwest::Client;
use std::path::Path;

/// HueBridge client
pub struct HueBridge<T: Into<String>> {
    client: Client,
    api_url: T,
    app_name: T,
    token: T,
}

impl<T> HueBridge<T>
where
    T: Into<String> + Default,
{
    /// Builds a new base client that can be used to eithe `register` a new application
    /// or
    /// ```
    /// let client = HueBridge::new("https://my-local-bridge-api.local");
    /// let light = light::Light::build().
    /// ```
    pub fn new(api_url: T) -> Self {
        let client = Client::new();

        Self {
            client,
            api_url,
            token: T::default(),
            app_name: T::default(),
        }
    }
    /// Register will register a new app
    pub async fn register(&self) -> Result<()> {
        Ok(())
    }

    async fn check_version(&self) -> Result<()> {
        Ok(())
    }

    pub fn verify_with_pem<P: AsRef<Path>>(&self) -> Result<()> {
        Ok(())
    }
}
