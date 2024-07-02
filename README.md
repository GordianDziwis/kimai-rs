[![Workflow Status](https://github.com/scattenlaeufer/kimai-rs/workflows/Rust%20checks/badge.svg)](https://github.com/scattenlaeufer/kimai-rs/actions?query=workflow%3A%22Rust+checks%22)

# kimai-rs

A client for the REST API of [Kimai](https://www.kimai.org/)

## Configuration

To be able to connect to Kimai, this crate needs some configuration. Those can
be loaded from `~/.config/kimai/config.toml`. This files should look as
follows:

```toml
host = "HOST_DOMAIN"
token = "TOKEN"
```

Tokens can be either stored in plain text in the configuration file, or read
from [`pass`](https://www.passwordstore.org/). For the later, the path within
`pass` needs to be stored in `pass_path` within the configuration file. Also
the `token` parameter needs to be omitted, since a plain text token takes
preferred to a token in pass.
