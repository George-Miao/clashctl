alias r := run

ui:
	cargo run --features ui

dev:
	cargo watch -x 'check --features ui' -s 'touch .trigger' > /dev/null & 
	cargo watch --no-gitignore -w .trigger -x 'run --features ui'

run args:
	cargo run --features ui {{ args }}

build:
	cargo run build --releaseq

add crate:
	cargo add {{ crate }} --upgrade patch --optional
