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
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Init,
    Standby,
    Powersave,
    Balance,
    Performance,
    Fast,
    Pedestal,
}

impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "init" => Self::Init,
            "standby" => Self::Standby,
            "powersave" => Self::Powersave,
            "balance" => Self::Balance,
            "performance" => Self::Performance,
            "fast" => Self::Fast,
            "pedestal" => Self::Pedestal,
            _ => return Err(anyhow!("illegal Mode")),
        })
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Self::Init => "init",
            Self::Standby => "standby",
            Self::Powersave => "powersave",
            Self::Balance => "balance",
            Self::Performance => "performance",
            Self::Fast => "fast",
            Self::Pedestal => "pedestal",
        }
        .into()
    }
}
