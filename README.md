# Rust telemetry example
This example shows how telemetry could work in Rust through procedural macros.

I did it just because. If you unironically use this somewhere, I wish all the worst for you. ❤️

## Running
```sh
# Run the HTTP server, listening at localhost:6969
cargo run --bin server

# Run the example program, calling a macro with the telemetry request
cargo run --bin example
```

## How does it work?
Procedural macros are contained in special crates that are compiled by the Rust toolchain into some kind of internal executables. Those are then executed whenever Rust needs to evaluate one of their macros. They seem to have access to the same system resources as rustc. This means, that they are capable of performing entire network requests.

This repository comes with 2 directories: server and client. Server is a simple HTTP server with telemetry endpoints, described below. Client contains a macro package and an example use of it.

## Server API
The HTTP server is simple and defines 2 GET endpoints:
 * **/** - the root contains a simple HTML website dumping all taken records.
 * **/telemetry/{1}/{2}/{3}/{4}** - registers a new telemetry record. Those records aren't saved to disk. Paramaters:
   - {1}: **crate_name** - name of the caller crate
   - {2}: **crate_version** - version of the caller crate
   - {3}: **os_name** - OS name
   - {4}: **ts_hash** - hash of the input TokenStream
