/* Copyright 2023 shadow3aaa@gitbub.com
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License. */
use std::fs;

use serde::{Deserialize, Serialize};

use super::POWER_CONFIG;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PowerConfig {
    #[serde(default)]
    pub features: Features,
    pub entry: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Features {
    pub strict: bool,
    pub pedestal: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            strict: true,
            pedestal: false,
        }
    }
}

pub fn get_powercfg() -> anyhow::Result<PowerConfig> {
    let json = fs::read_to_string(POWER_CONFIG)?;
    let json = serde_json::from_str(&json)?;

    Ok(json)
}

#[test]
fn parse_test() {
    let fas_rs = PowerConfig {
        features: Features {
            strict: true,
            pedestal: false,
        },
        entry: "/data/powercfg.sh".into(),
    };

    let test = r#"
    {
        "features": {
            "strict": true,
            "pedestal": false
        },
        "entry": "/data/powercfg.sh"
    }
    "#;

    assert_eq!(fas_rs, serde_json::from_str(test).unwrap());
}
