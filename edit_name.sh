#!/bin/bash

if [ "$1" = "--name" ]; then
	name=$2
else
	echo "帮助:"
	echo "自动修改名称模板"
	echo "很多地方, 比如Cargo.toml的模块name等和模块的module.prop以及可执行文件名都依赖此名称生成"
	echo "因为修改前用特殊标识占位, 只能修改一次, 修改完后脚本会自动删除"

	echo "示例: bash ./edit_name.sh --name foo"
fi

if [ "$name" = "" ]; then
	echo "未指定自定义名字"
	exit 1
fi

find . -not -name "edit_name.sh" -type f -exec sed -i "s/template_nha9nday6c/${name}/g" {} \;

echo "初始化仓库成功"
echo "更名: template_nha9nday6c -> $name"
echo "此脚本已移除, 如果还需更改项目名(不建议)只能手动修改"

rm edit_name.sh
