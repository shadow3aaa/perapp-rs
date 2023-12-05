#!/bin/bash
SHDIR="$(dirname $(readlink -f "$0"))"
SCRIPT=$SHDIR/script

case $1 in
build)
	source $SCRIPT/build.sh
	shift
	build $@
	;;
format | fmt)
	source $SCRIPT/format.sh
	format_codes
	;;
fix)
	source $SCRIPT/fix.sh
	fix_codes
	;;
help)
	echo "./make.sh:
    build:
        build and package module
        sugg: try ./make.sh build --help to get details
    format:
        format codes
    fix:
        fix codes"
	;;
*)
	echo Illegal parameter: $1 >&2
	echo Try \'./make.sh help\' >&2
	exit 1
	;;
esac
