# Clashctl

## About <a name = "about"></a>

Easy-to-use TUI & CLI to interact with [Clash](https://github.com/Dreamacro/clash) RESTful API.

## Screenshots <a name = "screenshots"></a>

### Status panel

![Status panel](https://imagedelivery.net/b21oeeg7p6hqWEI-IA5xDw/be2ffc2e-4193-4418-0d0f-b82624f0c800/public)

### Proxies panel

![Proxies panel](https://imagedelivery.net/b21oeeg7p6hqWEI-IA5xDw/0166f654-c5c2-4b0a-e401-8d5b93d3f500/public)

## Installing <a name = "installing"></a>

### From [`crates.io`](https://crates.io)

```bash
$ cargo install clashctl --vers 0.3.0-alpha.2 --all-features
```

### Compile from source

```bash
$ git clone https://github.com/George-Miao/clashctl.git
$ cd clashctl
$ cargo install --features "cli, ui" --path .
```

## Getting Started <a name = "getting_started"></a>

Use command without subcommands defaults to open TUI:

```bash
$ clashctl

# Equals

$ clashctl tui
```

Or use a subcommand to use the cli:

```bash
$ clashctl proxy list

---------------------------------------------------------
TYPE                DELAY   NAME
---------------------------------------------------------
selector            -       All

    URLTest         -       Auto-All
    ShadowsocksR    19      SomeProxy-1
    Vmess           177     SomeProxy-2
    Vmess           137     SomeProxy-3
    Shadowsocks     143     SomeProxy-4

---------------------------------------------------------
```

## Features <a name = "features"></a>

- Pretty terminal UI
- Change proxies
- Display proxies, with filter and sorting supported, in both plain and grouped mode
- Store and use multiple servers
- Generate completion script (by [clap_generate](https://crates.io/crates/clap_generate))
- Manage multiple servers

### Done & TODO <a name = "todo"></a>

- [ ] Cli
  - [x] Manage servers
  - [x] Sort proxies
  - [ ] More features
- [ ] TUI
  - [x] Status Panel
  - [x] Proxies Panel
    - [ ] Update proxy
    - [ ] Test latency
    - [ ] Sort by {Original, LatencyAsc, LatencyDsc, NameAsc, NameDsc}
  - [x] Rules Panel
  - [x] Connections Panel
    - [ ] Sort
  - [x] Log Panel
  - [x] Debug Panel
  - [ ] Config Panel
    - [ ] Update clash configs
    - [ ] Update clashctl configs
  - [ ] Search
  - [ ] (Maybe?) mouse support

## Prerequisites <a name = "prerequisites"></a>

You will need nightly rust environment (Cargo & rustc) to compile and install

## Usage <a name = "usage"></a>

### Use the TUI

- Use the cli to config servers (for now)
- Use number to navigate between tabs
- Space to hold the list (and therefor move the list)
- Arrow key to move the list under Hold mode
- [^d] open debug panel

### Use the CLI

```bash
$ clashctl -h
clashctl

George Miao <gm@miao.dev>

CLI used to interact with Clash RESTful API

USAGE:
    clashctl [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>      Path of config file. Default to ~/.config/clashctl/config.ron
    -h, --help                 Print help information
    -t, --timeout <TIMEOUT>    Timeout of requests, in ms [default: 2000]
    -v, --verbose              Verbosity. Default: INFO, -v DEBUG, -vv TRACE
    -V, --version              Print version information

SUBCOMMANDS:
    completion    Generate auto-completion scripts
    help          Print this message or the help of the given subcommand(s)
    proxy         Interacting with proxies
    server        Interacting with servers
```

### Use as a crate

```toml
# cargo.toml

[dependencies]
clashctl = "*"
```

Then in your project:

```rust
use clashctl::Clash;

fn main() {
  let clash = Clash::builder("http://example.com:9090").unwrap().build();
  println!("Clash version is {:?}", clash.get_version().unwrap())
}
```

## Development <a name = "development"></a>

`clashctl` comes with a [`justfile`](https://github.com/casey/just) to speed up your development.
Especially the command `just dev`, managed to reproduce the hot reload function in front-end development, with [`cargo-watch`](https://github.com/watchexec/cargo-watch).

### [`Just`](https://github.com/casey/just) commands

#### `just dev`

Hot reload development, auto reload on `cargo-check` approved changes, with all features enabled

#### `just ui`

Run UI only

#### `just run {{ Args }}`

Run with feature cli & ui

#### `just build`

Build in release mode with feature cli & ui

#### `just add`

Add an optional dependency, requires [`cargo-edit`](https://github.com/killercup/cargo-edit)

### Project structure

```bash
$ tree src -L 2
src
├── api.rs            # Clash API, with struct `Clash`, export by default
├── bin               # Binary dir
│   ├── cli.rs        # Cli only
│   ├── cli_ui.rs     # Both cli and ui
│   └── ui.rs         # Ui only
├── cli               # Feature `cli`, depends on clap
│   └── ...
├── error.rs          # Error
├── interactive       # Feature `interactive`, shared code of `cli` and `ui`
│   └── ...
├── lib.rs            # Lib entrance
├── model             # Models, export by default
│   └── ...
├── test              # Test codes
│   └── ...
└── ui                # Feature `ui`
    └── ...
```
