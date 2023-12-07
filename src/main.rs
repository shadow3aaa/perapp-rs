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
mod config;
mod mode;
mod powercfg;
mod topapp;

use std::{fs, path::Path, process::Command, thread, time::Duration};

use libc::pid_t;

use config::Config;
use mode::Mode;
use powercfg::get_powercfg;
use topapp::get_topapp;

pub(crate) const POWER_CONFIG: &str = "/data/powercfg.json";
pub(crate) const POWER_CONFIG_SHELL: &str = "/data/powercfg.sh";
const DEV_DIR: &str = "/dev/topapp_rs";

fn main() -> anyhow::Result<()> {
    let power_cfg = get_powercfg()?;
    let mut perapp_cfg = Config::new()?;
    let mut mode = Mode::Init;
    let (mut pid, mut pkg) = (-1, String::new());

    let dir = Path::new(DEV_DIR);
    let _ = fs::create_dir(dir);
    fs::write(dir.join("pkg"), &pkg)?;
    fs::write(dir.join("pid"), pid.to_string())?;

    exec_powercfg(pid, &pkg, mode)?;

    loop {
        thread::sleep(Duration::from_millis(1250));
        perapp_cfg.update()?;

        let (cur_pid, cur_pkg) = get_topapp().unwrap_or_default();

        fs::write(dir.join("pkg"), &pkg)?;
        fs::write(dir.join("pid"), pid.to_string())?;

        let mut cur_mode = perapp_cfg.mode(&cur_pkg);

        if power_cfg.features.pedestal {
            let status = "/sys/class/power_supply/battery/status";
            let status = fs::read_to_string(status)?;

            if status.trim() == "Charging" {
                cur_mode = Mode::Pedestal;
            }
        }

        if !device_wake()? {
            cur_mode = Mode::Standby;
        }

        if pkg != cur_pkg {
            pkg = cur_pkg;
            pid = cur_pid;

            if power_cfg.features.strict {
                mode = cur_mode;
                exec_powercfg(pid, &pkg, mode)?;
                continue;
            }
        }

        if cur_mode != mode {
            mode = cur_mode;
            exec_powercfg(pid, &pkg, mode)?;
        }
    }
}

fn exec_powercfg(pid: pid_t, pkg: &str, mode: Mode) -> anyhow::Result<()> {
    let arg = format!("{POWER_CONFIG_SHELL} {}", mode.to_string());
    Command::new("sh")
        .args(["-c", &arg])
        .envs([("pid", pid.to_string()), ("pkg", pkg.to_string())])
        .spawn()?;

    Ok(())
}

fn device_wake() -> anyhow::Result<bool> {
    let dump = Command::new("dumpsys")
        .args(["window", "policy"])
        .output()?;
    let dump = String::from_utf8_lossy(&dump.stdout).into_owned();

    Ok(dump.contains("screenState=SCREEN_STATE_ON"))
}
