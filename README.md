# Clashctl

## About <a name = "about"></a>

Easy-to-use command line tool to interact with [Clash](https://https://github.com/Dreamacro/clash) RESTful API.

## Getting Started <a name = "getting_started"></a>

```bash
$ clashctl proxy list

---------------------------------------------------------
TYPE            DELAY   NAME
---------------------------------------------------------
Selector        -       SelectorGroup
Direct          -       DIRECT
Reject          -       REJECT
URLTest         -       Auto-All
Selector        -       All
ShadowsocksR    19      SomeProxy-1
Vmess           177     SomeProxy-2
Vmess           137     SomeProxy-3
Shadowsocks     143     SomeProxy-4

```

## Features
- Change proxies
- Store and use multiple servers
- Generate completion script

### TODO
- [ ] Manage servers
- [ ] Sort proxies
- [ ] Inspect rules
- [ ] Status panel


## Installing

Since the project is not published yet, you can clone and compile from source manually:

### Compile from source

#### Prerequisites

You will need rust environment to compile and install

#### Install

```bash
$ git clone https://github.com/George-Miao/clashctl.git
$ cd clashctl
$ cargo install --features cli --path .
```

## MSRV
Minimum supported rust version is `1.56.0`

Test with cargo-msrv

## Usage <a name = "usage"></a>

### Use the CLI

```
clashctl 0.1.0

George Miao <gm@miao.dev>

CLI used to interact with Clash RESTful API

USAGE:
    clashctl [OPTIONS] <SUBCOMMAND>

OPTIONS:
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
clashctl = { version = "0.1.0", git = "https://github.com/George-Miao/clashctl.git" }
```

Then in your project: 

```rust
use clashctl::Clash;

#[tokio::test]
fn test() {
  let clash = Clash::builder("http://example.com:9090").unwrap().build();
  println!("Clash version is {:?}", clash.get_version().unwrap())
}
```