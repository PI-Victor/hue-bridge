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

use super::ResourceTypes;
use crate::HueBridge;
use crate::Result;
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
            endpoint: "clip/v2/resource/light".to_string(),
            bridge,
            ..Default::default()
        }
    }
    /// List all the available light resources
    pub async fn list(self) -> Result<Vec<LightPayload>> {
        let resp: LightResponse = self
            .bridge
            .fetch_resource(Method::GET, &self.endpoint)
            .await?;

        if let Some(err) = resp.error {
            Err(err[0].description.to_string().into())
        } else {
            Ok(resp.data)
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
struct LightResponse {
    data: Vec<LightPayload>,
    error: Option<Vec<ResourceErr>>,
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LightPayload {
    pub id: String,
    pub on: On,
    pub dimming: Dimming,
    pub dimming_delta: Option<DimmingDelta>,
    pub color_temperature: ColorTemperature,
    r#type: ResourceTypes,
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct On {
    pub on: bool,
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
struct ResourceErr {
    description: String,
}

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dimming {
    pub brightness: f32,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub mirek: u16,
}

#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ColorTemperatureDelta {
    pub action: Action,
    pub mirek_delta: u16,
}
