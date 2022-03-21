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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    endpoint: String,
    pub on: bool,
    pub dimming: Dimming,
    pub dimming_delta: DimmingDelta,
    pub color_temperature: ColorTemperature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimming {
    pub brigthness: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DimmingDelta {
    pub action: Action,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorTemperature {
    pub Mirek: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorTemperatureDelta {
    pub action: Action,
    pub mirek_delta: u16,
}
