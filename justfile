alias r := run

ui:
	cargo run --features ui --bin clashctl_ui

reset_terminal:
	pkill clashctl && stty sane && stty cooked

dev:
	cargo watch -x 'check --all-features' -s 'touch .trigger' > /dev/null & 
	cargo watch --no-gitignore -w .trigger -s 'pkill clashctl && stty sane' -x 'run --features "ui, cli"'

run args:
	cargo run --features "ui, cli" {{ args }}

build:
	cargo run build --release --features "ui, cli"

add crate:
	cargo add {{ crate }} --upgrade patch --optional

test args:
	cargo test --package clashctl --lib --all-features -- {{ args }} --nocapture