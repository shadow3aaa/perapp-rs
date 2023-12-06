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
use std::{
    collections::HashMap,
    process::Command,
    time::{Duration, Instant},
};

use libc::pid_t;

pub fn get_topapp() -> Option<(pid_t, String)> {
    let dump = dump()?;
    parse_topapp(&dump)
}

fn dump() -> Option<String> {
    let dump = Command::new("dumpsys")
        .args(["window", "visible-apps"])
        .output()
        .ok()?;

    Some(String::from_utf8_lossy(&dump.stdout).into_owned())
}

fn parse_topapp(dump: &str) -> Option<(pid_t, String)> {
    let pid = dump.lines().find(|l| l.contains("Session{"))?;
    let pid = pid.split_whitespace().nth(3)?;
    let pid = pid.split(':').next()?;
    let pid = pid.trim().parse().ok()?;

    let pkg = dump.lines().find(|l| l.contains("package="))?;
    let pkg = pkg.split_whitespace().nth(2)?;
    let pkg = pkg.split('=').nth(1)?.to_string();

    Some((pid, pkg))
}
