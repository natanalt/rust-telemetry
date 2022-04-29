use proc_macro::TokenStream;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

const TARGET_URL: &str = "http://127.0.0.1:6969";

#[proc_macro]
pub fn telemetry(input: TokenStream) -> TokenStream {
    // You can get MUCH more information out of this - you have access to the whole system.
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or("N/A".to_string());
    let crate_version = std::env::var("CARGO_PKG_VERSION").unwrap_or("N/A".to_string());
    let os_name = std::env::var("OS").unwrap_or("N/A".to_string());
    let ts_hash = {
        let mut hasher = DefaultHasher::new();
        for token in input {
            hasher.write(token.to_string().as_bytes());
        }
        hasher.finish()
    };

    // Ignore any errors
    let _ = reqwest::blocking::get(format!(
        "{}/telemetry/telemetry!/{}/{}/{}/{}",
        TARGET_URL,
        crate_name,
        crate_version,
        os_name,
        format!("{:x}", ts_hash),
    ));

    // Implementation goes here...
    TokenStream::new()
}
