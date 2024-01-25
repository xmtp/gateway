## XMTP Postal Service (XPS) Gateway

![XPS](xps.png)

This _postal service_ gateway implements the XMTP transport for registration, inbox and conversations.

[![codecov](https://codecov.io/gh/xmtp/xps-gateway/graph/badge.svg?token=HXZBPB9GIN)](https://codecov.io/gh/xmtp/xps-gateway)

## Quick Start (Development)

-   [READ THE DOCS](https://xmtp.github.io/xps-gateway)
-   [CONTRIBUTING](CONTRIBUTING.md)

### Dev Containers Development

This project supports containerized development. From Visual Studio Code Dev Containers extension

`Reopen in Container`

or

Command line build using docker

```bash
$ docker build . -t xps-gateway:1
```

## Testing (command line)

```bash
$ cargo test
```
