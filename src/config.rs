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
use std::{collections::HashMap, fs, str::FromStr, thread, time::Duration};

use inotify::{Inotify, WatchMask};

use crate::mode::Mode;

const CONFIG: &str = "/data/media/0/Android/perapp-rs/config.txt";

pub struct Config {
    inotify: Inotify,
    data: HashMap<String, Mode>,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let inotify = Inotify::init()?;
        inotify
            .watches()
            .add(CONFIG, WatchMask::CLOSE_WRITE | WatchMask::MODIFY)?;

        let conf = fs::read_to_string(CONFIG)?;
        let data = Self::parse_config(&conf);

        Ok(Self { inotify, data })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        let mut reparse = false;

        let mut buffer = [0; 1024];
        if let Ok(event) = self.inotify.read_events(&mut buffer) {
            if event.count() > 0 {
                reparse = true;
            }
        }

        if reparse {
            println!("reparse");
            loop {
                if let Ok(conf) = fs::read_to_string(CONFIG) {
                    self.data = Self::parse_config(&conf);
                    break;
                }

                thread::sleep(Duration::from_secs(1));
                eprintln!("Failed to read config, retrying…");
            }
        }

        let inotify = loop {
            let inotify = Inotify::init()?;
            if inotify
                .watches()
                .add(CONFIG, WatchMask::CLOSE_WRITE | WatchMask::MODIFY)
                .is_ok()
            {
                break inotify;
            }

            thread::sleep(Duration::from_secs(1));
            eprintln!("Failed to watch config, retrying…");
        };

        self.inotify = inotify;
        Ok(())
    }

    pub fn mode(&self, pkg: &str) -> Mode {
        if let Some(mode) = self.data.get(pkg).copied() {
            mode
        } else {
            self.data.get("default-mode").copied().unwrap()
        }
    }

    fn parse_config(conf: &str) -> HashMap<String, Mode> {
        conf.lines()
            .filter(|l| !l.trim().starts_with('#'))
            .filter(|l| !l.trim().starts_with("//"))
            .filter_map(|l| {
                let mut iter = l.split_whitespace();
                let pkg = iter.next()?.to_string();
                let mode = iter.next()?;
                let mode = Mode::from_str(mode).ok()?;

                Some((pkg, mode))
            })
            .collect()
    }
}
