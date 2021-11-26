alias r := run

ui:
	cargo run --features ui

dev:
	cargo watch -x 'check --features "ui, cli"' -s 'touch .trigger' > /dev/null & 
	cargo watch --no-gitignore -w .trigger -x 'run --features "ui, cli"'

run args:
	cargo run --features "ui, cli" {{ args }}

build:
	cargo run build --release --features "ui, cli"

add crate:
	cargo add {{ crate }} --upgrade patch --optional
