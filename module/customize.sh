#!/system/bin/sh
#
# Copyright 2023 shadow3aaa@gitbub.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
SKIPUNZIP=0
USERDIR=/sdcard/Android/perapp-rs

ui_print "如果使用了scene, 则不推荐使用此模块"
ui_print "配置文件在 $USERDIR"
ui_print "如果模块没有检测到powercfg, 则会自动关闭自己, 请确保powercfg存在"

mkdir $USERDIR
mv $MODPATH/config.toml $USERDIR/config.toml

set_perm_recursive $MODPATH 0 0 0755 0644
set_perm $MODPATH/perapp-rs 0 0 0755
