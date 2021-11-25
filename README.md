# Clashctl

## About <a name = "about"></a>

Easy-to-use TUI & CLI to interact with [Clash](https://github.com/Dreamacro/clash) RESTful API.

## Preview

<div style="display: flex;">
  <img src="https://imagedelivery.net/b21oeeg7p6hqWEI-IA5xDw/d187c9e0-5f27-4158-aae9-42a7762bcc00/public" />
  <img src="https://imagedelivery.net/b21oeeg7p6hqWEI-IA5xDw/cee94606-ed5c-4cd3-ecd0-0afa4b6beb00/public" />
</div>

## Getting Started <a name = "getting_started"></a>

Use command without subcommands defaults to open TUI:

```bash
$ clashctl

# Equals

$ clashctl tui
```

Or use a subcommand:

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

## Features

- Pretty terminal UI
- Change proxies
- Display proxies, with filter and sorting supported, in both plain and grouped mode
- Store and use multiple servers
- Generate completion script (by [clap_generate](https://crates.io/crates/clap_generate))
- Manage multiple servers

### Done & TODO

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

## Installing

### Use Cargo

```bash
$ cargo install clashctl --vers 0.3.0-alpha.2 --all-features
```

### Compile from source

```bash
$ git clone https://github.com/George-Miao/clashctl.git
$ cd clashctl
$ cargo install --features "cli, ui" --path .
```

## Prerequisites

You will need nightly rust environment (Cargo & rustc) to compile and install

## Usage <a name = "usage"></a>

### Use the TUI

- Use number to navigate between tabs
- Space to hold the list (and therefor move the list)
- Arrow key to move the list under Hold mode
- ^D open debug panel

### Use the CLI

```
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

fn test() {
  let clash = Clash::builder("http://example.com:9090").unwrap().build();
  println!("Clash version is {:?}", clash.get_version().unwrap())
}
```
