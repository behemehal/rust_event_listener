#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]
#![doc(html_root_url = "https://docs.rs/rust_event_listener/0.1.0")]

//!# rust_event_listener
//!NodeJS like Event Listener library for rust!
//!
//!## Usage
//!
//! ```
//! use rust_event_listener::EventListener;
//!
//! fn main() {
//!    let mut emitter = EventListener::new();
//!    //Set max listeners
//!    emitter.set_max_listeners(10);
//!
//!    //Add listener
//!    emitter.on("test",  Box::new(|name, d| {
//!        println!("Emited: {} {:#?}", name, d);
//!    }));
//!    emitter.emit("test", "1".to_string());
//! }
//! ```
//! You can find more examples [here](https://github.com/behemehal/Menemen/tree/main/examples)

/// Listener utilities
pub mod listener;

/// Event interface
pub struct Event {
    /// Event name
    pub name: String,
    /// Event listeners
    pub data: Vec<crate::listener::Listener>,
}

/// EventListener
pub struct EventListener {
    /// All events
    pub events: Vec<Event>,
    /// Max listeners
    max_listeners: usize,
}

impl EventListener {
    /// Create a new EventListener
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut emitter = EventListener::new();
    /// ```
    pub fn new() -> Self {
        EventListener {
            events: vec![
                Event {
                    name: "newListener".to_string(),
                    data: vec![],
                },
                Event {
                    name: "removeListener".to_string(),
                    data: vec![],
                },
            ],
            max_listeners: 10,
        }
    }

    /// Sets the maximum number of listeners that can be registered.
    /// ## Parameters
    /// `max_listeners` - The maximum number of listeners that can be registered.
    pub fn set_max_listeners(&mut self, max_listeners: usize) {
        self.max_listeners = max_listeners;
    }

    /// Get max listeners for this EventListener
    /// ## Returns
    /// [`usize`]
    pub fn get_max_listeners(&self) -> usize {
        self.max_listeners
    }

    /// Add a new listener to the event
    /// ## Parameters
    /// * `name` - The name of the event
    /// * `callback` - The callback function
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut emitter = EventListener::new();
    /// emitter.on("test", Box::new(|name, data| {
    ///    println!("{}", data);
    /// }));
    /// ```
    pub fn on(&mut self, name: &str, callback: crate::listener::ListenerCallback) {
        let event = Event {
            name: name.to_string(),
            data: vec![],
        };

        if self.events.iter().find(|x| x.name == name).is_none() {
            self.events.push(event);
        }
        if self.max_listeners == 0
            || self
                .events
                .iter()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .len()
                < self.max_listeners
        {
            self.events
                .iter_mut()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .push(crate::listener::Listener {
                    rtype: crate::listener::ListenerTypes::On,
                    callback,
                });
        } else {
            panic!("Max listeners reached");
        }
    }

    /// Add a listener that will be called only once
    /// ## Parameters
    /// * `name` - The name of the event
    /// * `callback` - The callback function
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut event_listener = EventListener::new();
    /// event_listener.once("test", Box::new(|name, data| {
    ///    println!("{}", data);
    /// }));
    /// ```
    pub fn once(&mut self, name: &str, callback: crate::listener::ListenerCallback) {
        if self.events.iter().find(|x| x.name == name).is_none() {
            self.events.push(Event {
                name: name.to_string(),
                data: vec![],
            });
        }
        if self.max_listeners == 0
            || self
                .events
                .iter()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .len()
                < self.max_listeners
        {
            self.events
                .iter_mut()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .push(crate::listener::Listener {
                    rtype: crate::listener::ListenerTypes::Once,
                    callback,
                });
        } else {
            panic!("Max listeners reached");
        }
    }

    /// Get existing events
    /// ## Returns
    /// [`Vec<&Event>`]
    pub fn get_events(&self) -> Vec<&Event> {
        self.events.iter().map(|x| x).collect::<Vec<_>>()
    }

    /// Get existing event names
    /// ## Returns
    /// [`Vec<String>`]
    pub fn get_event_names(&self) -> Vec<String> {
        self.events.iter().map(|x| x.name.clone()).collect()
    }

    /// Get all existent listeners of event
    /// ## Parameters
    /// * `name` - The name of the event
    /// ## Returns
    /// [`Vec<&Listener>`]
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut event_listener = EventListener::new();
    /// event_listener.on("test", Box::new(|name, data| {
    ///  println!("test: {:?}", data);
    /// }));
    /// event_listener.get_listeners("test").iter().for_each(|x| {
    ///  println!("{:?}", x);
    /// });
    /// ```
    pub fn get_listeners(&self, name: &str) -> Vec<&crate::listener::Listener> {
        self.events
            .iter()
            .find(|x| x.name == name)
            .unwrap()
            .data
            .iter()
            .map(|x| x)
            .collect::<Vec<_>>()
    }

    /// Remove all listeners of event
    /// ## Parameters
    /// * `name` - The name of the event
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut event_listener = EventListener::new();
    /// event_listener.on("test", Box::new(|name, data| {
    ///  println!("test: {:?}", data);
    /// }));
    /// event_listener.remove_all_listeners("test");
    /// ```
    /// ## Returns
    /// [`bool`] - `true` if the event was removed, `false` if it wasn't
    pub fn remove_all_listeners(&mut self, name: &str) -> bool {
        if self.events.iter().find(|x| x.name == name).is_none() {
            return false;
        }
        self.events
            .iter_mut()
            .find(|x| x.name == name)
            .unwrap()
            .data
            .clear();
        true
    }

    /// Emit an event
    /// ## Parameters
    /// * `name` - The name of the event
    /// * `data` - The data to pass to the listeners
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut event_listener = EventListener::new();
    /// event_listener.on("test", Box::new(|name, data| {
    ///  println!("test: {:?}", data); // test: test
    /// }));
    /// event_listener.emit("test", "test".to_string());
    /// ```
    /// ## Panics
    /// If the event doesn't exist
    pub fn emit(&mut self, name: &str, data: String) {
        if self.events.iter().find(|x| x.name == name).is_none() {
            panic!("Event doesn't exist");
        }
        for i in &self.events {
            if i.name == name {
                for j in &i.data {
                    (j.callback)(name.to_string(), data.clone());
                }
            }
        }
    }
}
