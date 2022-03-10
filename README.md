# Rust Event Listener

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