# kimai-rs

A client for the REST API of [Kimai](https://www.kimai.org/)

## Configuration

To be able to connect to Kimai, this crate needs some configuration. Those can
be loaded from `~/.config/kimai/config.toml`. This files should look as
follows:

```toml
host = "HOST_DOMAIN"
user = "USERNAME"
password = "PASSWORD"
```

Passwords can be either stored in plain text in the configuration file, or read
from [`pass`](https://www.passwordstore.org/). For the later, the path within
`pass` needs to be stored in `pass_path` within the configuration file. Also
the `password` parameter needs to be omitted, since a plain text password takes
preferred to a password in pass.
