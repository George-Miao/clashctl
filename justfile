alias r := run
alias b := build
alias d := dev

ui:
	cargo run --features ui --bin clashctl_ui

cli:
	cargo run --features cli --bin clashctl_cli

reset_terminal:
	pkill clashctl && stty sane && stty cooked

dev:
	cargo watch -x 'check  --features "ui cli" > /dev/null 2>&1 ' -s 'touch .trigger' > /dev/null & 
	cargo watch --no-gitignore -w .trigger -x 'run --features "ui cli"'

run args:
	cargo run --features "ui, cli" {{ args }}

build:
	cargo build --release --features="cli ui"

add crate:
	cargo add {{ crate }} --upgrade patch --optional

test args:
	cargo test --package clashctl --lib --all-features -- {{ args }} --nocapture