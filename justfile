alias r := run

ui:
	cargo run --features ui

dev:
	cargo watch -w src -x 'check --features ui >> /dev/null 2>&1' -s 'touch .trigger' >> /dev/null 2>&1 &
	cargo watch --no-gitignore -w .trigger -x 'run --features ui --bin clashctl_ui'

run args:
	cargo run --features ui {{ args }}

build:
	cargo run build --releaseq

add crate:
	cargo add {{ crate }} --upgrade patch --optional