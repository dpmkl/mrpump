# mrpump

`mrpump` is a simple, lightweight TCP socket proxy written in Rust. It allows you to forward TCP traffic from one address to another, with built-in logging and metrics.

## Features

- **Simple Configuration**: Easy to set up using a YAML configuration file.
- **Systemd Integration**: Comes with a systemd service file for easy management on Linux.

## Installation

### From Source

Ensure you have Rust and Cargo installed, then:

```bash
cargo build --release
```

### Using the Install Script

You can use the provided `install.sh` to install the binary, configuration, and systemd service:

```bash
./dist/install.sh
```

## Configuration

`mrpump` looks for a configuration file in the following locations (in order):
1. `/etc/mrpump/config.yaml`
2. `~/.config/mrpump/config.yaml`
3. `./config.yaml`

Example `config.yaml`:

```yaml
sockets:
  web:
    listen: "0.0.0.0:8080"
    target: "127.0.0.1:80"
  db:
    listen: "127.0.0.1:5432"
    target: "10.0.0.5:5432"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
