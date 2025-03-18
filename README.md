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
  *(Note: At present, the write functionality can be added by modifying the global store.)*

## Installation

Add **zirv-config** as a dependency in your project's `Cargo.toml`:

```sh
cargo add zirv-config
```

## Usage

Before using the macros, initialize the global configuration store. In your application’s startup code, call:

```rust
// Initialize the global configuration
zirv_config::config::init_config();
```

### Registering Configuration Blocks
Register a configuration block under a namespace (for example, "server") using the provided register_config! macro. Any type that implements Serialize can be used.

```rust
use zirv_config::register_config;
use serde::Serialize;

#[derive(Serialize)]
struct ServerConfig {
    port: u16,
    host: String,
}

fn main() {
    // Ensure the configuration store is initialized
    zirv_config::config::init_config();

    // Register your server configuration
    let server_config = ServerConfig { port: 3000, host: "0.0.0.0".into() };
    register_config!("server", server_config);
}
```

### Reading Configuration Values
Retrieve the full configuration or a specific key using the read_config! macro. You can also request to deserialize a particular value into a desired type.

```rust
use zirv_config::read_config;

fn display_config() {
    // Retrieve the full configuration as a serde_json::Value
    let full_config = read_config!();
    println!("Full config: {:?}", full_config);

    // Get a specific configuration value as JSON
    if let Some(port) = read_config!("server.port") {
        println!("Server port (JSON): {}", port);
    }

    // Get a specific configuration value and convert it to the desired type
    if let Some(port) = read_config!("server.port", u16) {
        println!("Server port as u16: {}", port);
    }
}
```

### License
This project is licensed under the MIT license.

### Contributing
Contributions are welcome! Please refer to the [contributing guidelines](CONTRIBUTING.md) for more information.