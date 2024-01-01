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
use std::{collections::HashMap, str::FromStr};

use serde::Deserialize;
use toml::value::Array;

use crate::mode::Mode;

#[derive(Debug, PartialEq)]
pub struct Data {
    pub onscreen_mode: Mode,
    pub offscreen_mode: Mode,
    list: HashMap<String, Mode>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlRaw {
    pub global: Global,
    pub powersave: List,
    pub balance: List,
    pub performance: List,
    pub fast: List,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Global {
    pub offscreen_mode: String,
    pub onscreen_mode: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct List {
    pub list: Array,
}

impl Data {
    pub fn new(raw: &str) -> anyhow::Result<Self> {
        let raw: TomlRaw = toml::from_str(raw)?;

        let global = raw.global;
        let onscreen_mode = Mode::from_str(&global.onscreen_mode)?;
        let offscreen_mode = Mode::from_str(&global.offscreen_mode)?;

        let mut list = HashMap::new();

        for pkg in raw.powersave.list.iter().filter_map(|p| p.as_str()) {
            list.insert(pkg.to_string(), Mode::Powersave);
        }

        for pkg in raw.balance.list.iter().filter_map(|p| p.as_str()) {
            list.insert(pkg.to_string(), Mode::Balance);
        }

        for pkg in raw.performance.list.iter().filter_map(|p| p.as_str()) {
            list.insert(pkg.to_string(), Mode::Performance);
        }

        for pkg in raw.fast.list.iter().filter_map(|p| p.as_str()) {
            list.insert(pkg.to_string(), Mode::Fast);
        }

        Ok(Self {
            onscreen_mode,
            offscreen_mode,
            list,
        })
    }

    pub fn get(&self, pkg: &str) -> Option<Mode> {
        self.list.get(pkg).copied()
    }
}

#[test]
fn parse_test() {
    let mut list = HashMap::new();
    list.insert("apple".to_string(), Mode::Powersave);
    list.insert("orange".to_string(), Mode::Balance);
    list.insert("banana".to_string(), Mode::Performance);
    list.insert("pear".to_string(), Mode::Fast);

    let target = Data {
        onscreen_mode: Mode::Balance,
        offscreen_mode: Mode::Powersave,
        list,
    };

    let test = r#"
    [global]
    onscreen_mode = "balance"
    offscreen_mode = "powersave"
    
    [powersave]
    list = ["apple"]
    
    [balance]
    list = ["orange"]
    
    [performance]
    list = ["banana"]
    
    [fast]
    list = ["pear"]
    "#;

    let test = Data::new(test).unwrap();
    assert_eq!(test, target);
}
