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

use super::ResourceTypes;
use crate::{HueBridge, Result};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
#[derive(Debug, Default)]
/// <https://developers.meethue.com/develop/hue-api-v2/api-reference/#resource_light>
pub struct Light {
    endpoint: String,
    bridge: HueBridge,
}

impl Light {
    /// Light resource constructor.
    pub fn new(bridge: HueBridge) -> Self {
        Self {
            endpoint: String::from("clip/v2/resource/light"),
            bridge,
            ..Default::default()
        }
    }
    /// List all the available light resources
    pub async fn list(&self) -> Result<Vec<LightPayload>> {
        let resp: LightResponse = self
            .bridge
            .do_request(Method::GET, &self.endpoint, None)
            .await?;

        if let Some(err) = resp.error {
            Err(err[0].description.to_string().into())
        } else {
            Ok(resp.data)
        }
    }
    /// Set light resource by id
    pub async fn set(&self, light: LightPayload) -> Result<LightResponse> {
        Ok(self
            .bridge
            .do_request(Method::PUT, &self.endpoint, None)
            .await?)
    }
    /// Get a light resource by id
    pub async fn get(&self, id: &str) -> Result<LightPayload> {
        let resp: LightResponse = self
            .bridge
            .do_request(
                Method::GET,
                format!("{}/{id}", &self.endpoint).as_str(),
                None,
            )
            .await?;

        if let Some(err) = resp.error {
            Err(err[0].description.to_string().into())
        } else {
            let data = resp.data.first().unwrap().to_owned();
            Ok(data)
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct LightResponse {
    data: Vec<LightPayload>,
    error: Option<Vec<ResourceErr>>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct LightPayload {
    pub id: String,
    pub metadata: Meta,
    pub on: On,
    pub dimming: Dimming,
    pub dimming_delta: Option<DimmingDelta>,
    pub color_temperature: ColorTemperature,
    r#type: ResourceTypes,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct Meta {
    name: String,
    archetype: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct On {
    pub on: bool,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct ResourceErr {
    description: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Serialize, Deserialize, Builder)]
pub struct Dimming {
    pub brightness: f32,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Up,
    Down,
    Stop,
}

impl Default for Action {
    fn default() -> Self {
        Action::Up
    }
}

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DimmingDelta {
    pub action: Option<Action>,
}

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ColorTemperature {
    pub mirek: Option<u16>,
}
