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

#![warn(missing_docs)]

//! This crate provides asynchronous API bindings for
//! [Hue Bridge CLIP API](https://developers.meethue.com/develop/hue-api-v2/).  
//! NOTE: You need to be registered and signed in to see the documentation.  
//!
//! In order to develop with this crate **you need physical access to a Hue Bridge**. The security
//! of the Hue Bridge revolves around pressing the button on the device to register an "application"
//! and get a unique application identifier back.  

/// Contains all CLIP API resource types
pub mod api;
pub use api::resources::Bridge;
pub use api::resources::Device;
pub use api::resources::Light;

/// Contains the hue bridge client
pub mod huebridge;
pub use huebridge::HueBridge;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
