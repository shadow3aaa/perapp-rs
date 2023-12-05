#!/bin/bash
format_codes() {
	find -type f -name "*.sh" -not -path ".git/*" -exec shfmt -w -s {} \;
	cargo fmt -v
}
