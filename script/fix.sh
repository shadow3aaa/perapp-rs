#!/bin/bash
fix_codes() {
	local FIX_COMMAND='cargo clippy --fix --allow-dirty --allow-staged'

	cd $SHDIR && $FIX_COMMAND
}
