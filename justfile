alias r := run
alias b := build
alias d := dev

ui *args:
	cargo run -p clashctl-tui -- {{ args }}

cli *args:
	cargo run -p clashctl-cli -- {{ args }}

reset_terminal:
	pkill clashctl && stty sane && stty cooked

dev:
	cargo watch -x 'check -p clashctl-tui > /dev/null 2>&1 ' -s 'touch .trigger' > /dev/null & 
	cargo watch --no-gitignore -w .trigger -x 'run -p clashctl-tui'

run *args:
	cargo run -p clashctl-bin -- {{ args }}

build:
	cargo build --release

test *args:
	cargo test -- {{ args }} --nocapture