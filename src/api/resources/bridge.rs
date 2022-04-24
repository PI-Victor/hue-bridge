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
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

const MINIMUM_BRIDGE_VERSION: u64 = 1948086000;

#[derive(Debug, Clone)]
struct MinimumSwVersionError;

impl Error for MinimumSwVersionError {}

impl fmt::Display for MinimumSwVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hue Bridge SW Version smaller than: {}",
            MINIMUM_BRIDGE_VERSION
        )
    }
}

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub name: String,
    pub datastoreversion: String,
    pub swversion: String,
    pub apiversion: String,
    pub mac: String,
    pub bridgeid: String,
    pub factorynew: bool,
    pub modelid: String,
}

impl Bridge {
    pub(crate) fn check_version(&self) -> Result<()> {
        let bridge_version = &self.swversion.parse::<u64>().unwrap();
        if bridge_version < &MINIMUM_BRIDGE_VERSION {
            Err(MinimumSwVersionError)?;
        }

        Ok(())
    }
}
