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
mod data;

use std::{fs, thread, time::Duration};

use inotify::{Inotify, WatchMask};

use anyhow::anyhow;

use crate::mode::Mode;
use data::Data;

const CONFIG: &str = "/data/media/0/Android/perapp-rs/config.toml";

pub struct Config {
    inotify: Inotify,
    data: Data,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let inotify = Inotify::init()?;
        inotify
            .watches()
            .add(CONFIG, WatchMask::CLOSE_WRITE | WatchMask::MODIFY)?;

        let conf = fs::read_to_string(CONFIG)?;
        let data = Data::new(&conf)?;

        Ok(Self { inotify, data })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        let _reparse = false;

        let mut buffer = [0; 1024];
        if let Ok(event) = self.inotify.read_events(&mut buffer) {
            if event.count() > 0 {
                self.reparse()?;
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

    pub fn reparse(&mut self) -> anyhow::Result<()> {
        let mut counter = 0;
        loop {
            let Ok(conf) = fs::read_to_string(CONFIG) else {
                counter += 1;

                if counter > 10 {
                    return Err(anyhow!("To many retries!"));
                }

                thread::sleep(Duration::from_secs(1));
                eprintln!("Failed to read config, retrying…");
                continue;
            };

            if let Ok(data) = Data::new(&conf) {
                self.data = data;
                return Ok(());
            }

            counter += 1;

            if counter > 10 {
                return Err(anyhow!("To many retries!"));
            }

            thread::sleep(Duration::from_secs(1));
            eprintln!("Failed to parse config, retrying…");
        }
    }

    pub fn mode(&self, pkg: &str, onscreen: bool) -> Mode {
        if onscreen {
            if let Some(mode) = self.data.get(pkg) {
                mode
            } else {
                self.data.onscreen_mode
            }
        } else {
            self.data.offscreen_mode
        }
    }
}
