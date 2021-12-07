# Clashctl Core

Lib for interacting with Clash RESTful API. This crate does not contain binary. For more information, check [clashctl](https://github.com/George-Miao/clashctl), a CLI & TUI tool built with this crate.

## RESTful API Methods

Functions of `Clash`

| Function Name             | Method | Endpoint                             |
| ------------------------- | ------ | ------------------------------------ |
| `get_version`             | GET    | /logs                                |
| `get_traffic`             | GET    | /traffic                             |
| `get_version`             | GET    | /version                             |
| `get_configs`             | GET    | /config                              |
| **TODO**                      | PUT    | /config                              |
| **TODO**                      | PATCH  | /config                              |
| `get_proxies`             | GET    | /proxies                             |
| `get_proxy`               | GET    | /proxies/:name                       |
| `set_proxygroup_selected` | PUT    | /proxies/:name                       |
| `get_proxy_delay`         | GET    | /proxies/:name/delay                 |
| `get_rules`               | GET    | /rules                               |
| `get_connections`         | GET    | /connections                         |
| **TODO**                      | DELETE | /connections                         |
| **TODO**                      | DELETE | /connections/:id                     |
| **TODO**                      | GET    | /providers/proxies                   |
| **TODO**                      | GET    | /providers/proxies/:name             |
| **TODO**                      | PUT    | /providers/proxies/:name             |
| **TODO**                      | GET    | /providers/proxies/:name/healthcheck |