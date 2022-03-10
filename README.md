# Rust Event Listener

[![Crates.io Version](https://img.shields.io/crates/v/rust_event_listener?logo=rust)](https://crates.io/crates/rust_event_listener)
[![Documentation](https://docs.rs/rust_event_listener/badge.svg)](https://docs.rs/rust_event_listener)

NodeJS like Event Listener library for rust!

```rust
    //Create new emitter
    let mut emitter = EventListener::new();

    //Set max listeners
    emitter.set_max_listeners(10);

    //Add listener
    emitter.on("test", Box::new(|name, d| {
        println!("Emited: {} {:#?}", name, d);
    }));

    emitter.emit("test", 1);
```

## Examples

You can find examples [here](https://github.com/behemehal/rust_event_listener/tree/main/examples)
