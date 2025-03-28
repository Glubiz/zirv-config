//! zirv-config library
//!
//! Provides an expandable configuration system where configuration is built up
//! from multiple subsystems (such as `server`, `logging`, etc.). The configuration
//! can be accessed as a whole or by specific keys using the `read_config!` macro.

pub mod config;

#[macro_export]
/// Retrieves the configuration from the global store.
/// 
/// Usage:
/// 
/// - `read_config!()` returns the entire configuration as a `serde_json::Value`.
/// - `read_config!("some.key")` returns an `Option<serde_json::Value>` for the specified dot-separated key.
/// 
/// # Examples
/// 
/// ```rust
/// # use zirv_config::read_config;
/// // Get full config
/// let full_config = read_config!();
/// println!("Config: {:?}", full_config);
///
/// // Get a specific key, e.g., "server.port"
/// if let Some(port) = read_config!("server.port") {
///     println!("Server port: {}", port);
/// }
/// ```
macro_rules! read_config {
    () => {
        $crate::config::get_config()
    };
    ($key:expr) => {
        $crate::config::get_config_by_key($key)
    };
}

#[macro_export]
/// Registers a configuration block under a given namespace.
///
/// This macro is a thin wrapper around the underlying
/// `config::register_config(namespace, config)` function.
///
/// # Examples
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
macro_rules! register_config {
    ($namespace:expr, $config:expr) => {{
        $crate::config::register_config($namespace, $config);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use serde::Serialize;

    // A dummy configuration struct for testing.
    #[derive(Serialize)]
    struct DummyConfig {
        port: u16,
        host: String,
    }

    /// Before running tests, we initialize the global configuration.
    /// Note: Since GLOBAL_CONFIG is a OnceLock, once it's set it cannot be cleared.
    /// These tests assume a fresh process or that they run serially.
    fn setup() {
        // Force initialization.
        config::init_config();
    }

    #[test]
    fn test_register_and_read_full_config() {
        setup();
        // Register a dummy server configuration.
        register_config!("server", DummyConfig { 
            port: 3000, 
            host: "0.0.0.0".to_string() 
        });
        
        // Retrieve the full configuration.
        let full = read_config!();
        // It should be a JSON object containing a key "server".
        if let Value::Object(map) = full {
            assert!(map.contains_key("server"), "Expected key 'server' not found");
            if let Some(Value::Object(server_obj)) = map.get("server") {
                // Verify the values.
                assert_eq!(server_obj.get("port").unwrap(), &json!(3000));
                assert_eq!(server_obj.get("host").unwrap(), &json!("0.0.0.0"));
            } else {
                panic!("'server' is not an object");
            }
        } else {
            panic!("Global config is not an object");
        }
    }

    #[test]
    fn test_read_config_by_key() {
        setup();
        // Assume "server" was registered in a previous test.
        // Retrieve a specific key:
        let port = read_config!("server.port");
        assert_eq!(port, Some(json!(3000)));

        let host = read_config!("server.host");
        assert_eq!(host, Some(json!("0.0.0.0")));

        // Request a non-existent key:
        let missing = read_config!("server.nonexistent");
        assert!(missing.is_none());
    }
}
