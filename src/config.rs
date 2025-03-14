//! Module for managing configuration.
//!
//! The global configuration is stored as a JSON object. Each subsystem (e.g. server, logging)
//! can register its configuration under a specific namespace (e.g. "server", "logging").
//! The library provides functions to read the full configuration or fetch a value using a dot-separated key.

use serde::Serialize;
use std::sync::OnceLock;
use serde_json::{Value, Map};

/// Global configuration store, as a JSON object wrapped in a Mutex for mutable access.
static GLOBAL_CONFIG: OnceLock<std::sync::Mutex<Map<String, Value>>> = OnceLock::new();

/// Initializes the global configuration as an empty JSON object.
/// This should be called once early in the application startup.
pub fn init_config() {
    GLOBAL_CONFIG.get_or_init(|| std::sync::Mutex::new(Map::new()));
}

/// Registers a configuration block under a given namespace.
///
/// # Arguments
///
/// * `namespace` - The key under which to register the configuration (for example, "server").
/// * `config` - The configuration data (which must implement `Serialize`).
///
/// # Example
///
/// ```rust
/// # use zirv_config::register_config;
/// #[derive(serde::Serialize)]
/// struct ServerConfig {
///     port: u16,
///     host: String,
/// }
///
/// let server_config = ServerConfig { port: 3000, host: "0.0.0.0".to_string() };
/// register_config!("server", server_config);
/// ```
pub fn register_config<T: Serialize>(namespace: &str, config: T) {
    let global = GLOBAL_CONFIG.get_or_init(|| std::sync::Mutex::new(Map::new()));
    let value = serde_json::to_value(config).expect("Serialization failed");
    let mut guard = global.lock().expect("Mutex poisoned");
    guard.insert(namespace.to_string(), value);
}

pub fn get_config() -> Value {
    if let Some(global) = GLOBAL_CONFIG.get() {
        let guard = global.lock().expect("Mutex poisoned");
        Value::Object(guard.clone())
    } else {
        Value::Object(Map::new())
    }
}

/// Retrieves a configuration value given a dot-separated key path (e.g., "server.port").
///
/// If the key is not found, returns `None`.
///
/// # Examples
///
/// ```rust
/// # use zirv_config::read_config;
/// // Get full config
/// let full_config = read_config!();
/// println!("Config: {:?}", full_config);
/// ```
pub fn get_config_by_key(key: &str) -> Option<Value> {
    let mut current = get_config();
    for part in key.split('.') {
        match current {
            Value::Object(map) => {
                if let Some(v) = map.get(part) {
                    current = v.clone();
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(serde::Serialize)]
    struct DummyConfig {
        port: u16,
        host: String,
    }

    #[test]
    fn test_register_and_get_full_config() {
        // Initialize global config.
        init_config();

        // Register two configuration blocks.
        register_config(
            "server",
            DummyConfig {
                port: 3000,
                host: "0.0.0.0".into()
            },
        );

        register_config("logging",
            json!({
                "level": "info",
                "file": "app.log",
            }),
        );

        let config = get_config();
        if let Value::Object(map) = config {
            assert!(map.contains_key("server"));
            assert!(map.contains_key("logging"));
        } else {
            panic!("Global config is not an object");
        }
    }

    #[test]
    fn test_get_config_by_key() {
        // Clear and reinitialize the global config.
        // (For testing, we can simulate a new initialization by using a new OnceLock instance.)
        // Note: In real usage, GLOBAL_CONFIG is static and persistent across calls.
        init_config();
        register_config(
            "server",
            DummyConfig {
                port: 3000,
                host: "0.0.0.0".into()
            },
        );

        let port = get_config_by_key("server.port");
        assert_eq!(port, Some(json!(3000)));

        let host = get_config_by_key("server.host");
        assert_eq!(host, Some(json!("0.0.0.0")));

        // Non-existent key returns None.
        let missing = get_config_by_key("server.nonexistent");
        assert!(missing.is_none());
    }
}
