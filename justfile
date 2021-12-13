alias r := run
alias b := build
alias d := dev

run *args:
	cargo run -p clashctl -- {{ args }}

ui *args:
	cargo run -p clashctl-tui -- {{ args }}

reset_terminal:
	pkill clashctl && stty sane && stty cooked

dev:
	cargo watch -x 'check -p clashctl-tui > /dev/null 2>&1 ' -s 'touch .trigger' > /dev/null & 
	cargo watch --no-gitignore -w .trigger -x 'run -p clashctl-tui'

build:
	cargo build --release

release os: build
	#!/usr/bin/env bash
	pushd target/release
	rm clashctl*.d
	mv clashctl-tui* clashctl-tui-{{ os }}
	mv clashctl* clashctl-{{ os }}
	popd
	
test *args:
	cargo test -- {{ args }} --nocapture