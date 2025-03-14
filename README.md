# zirv-config

**zirv-config** is an expandable configuration library for Rust backend applications. It provides a global, mutable configuration store that aggregates settings from multiple subsystems (e.g. server, logging, etc.). Developers can register configuration blocks under namespaces, and later retrieve or update the configuration using convenient macros.

## Features

- **Global Configuration Store:**  
  The configuration is maintained as a JSON object in a global store. This store is expandable—subsystems can register their configuration data under a unique namespace.

- **Environment Integration:**  
  Automatically loads environment variables (and optionally a `.env` file via `dotenvy`) to initialize configuration values.

- **Dynamic Registration:**  
  Easily register new configuration blocks using the `register_config!` macro.

- **Flexible Access:**  
  Retrieve the entire configuration or a specific configuration value by using dot-separated keys with the `read_config!` macro.

- **Runtime Updates:**  
  Update configuration values at runtime using the `write_config!` macro.

## Installation

Add **zirv-config** as a dependency in your project's `Cargo.toml`:

```sh
cargo add zirv-config
