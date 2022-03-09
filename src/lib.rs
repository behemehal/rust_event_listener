use core::fmt::Debug;
use std::sync::Arc;
use listener::{Listener, ListenerTypes, ListenerCallback};
pub mod listener;

trait ListenerType {}

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub data: Vec<Listener>,
}

pub struct EventListener {
    pub listeners: Vec<Event>,
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
        let newListener = Event {
            name: "newListener".to_string(),
            data: vec![],
        };


        let removeListener = Event {
            name: "removeListener".to_string(),
            data: vec![],
        };
        
        EventListener {
            listeners: vec![
                newListener,
                removeListener,
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
    /// emitter.on("test", |data| {
    ///    println!("{}", data);
    /// });
    /// ```
    pub fn on<T: Debug + 'static>(
        &mut self,
        name: &str,
        callback: ListenerCallback,
    ) {
        let event = Event {
            name: name.to_string(),
            data: vec![],
        };

        if self.listeners.iter().find(|x| x.name == name).is_none() {
            self.listeners.push(event);
        }
        if self.max_listeners == 0
            || self
                .listeners
                .iter()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .len()
                < self.max_listeners
        {
            self.listeners
                .iter_mut()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .push(Listener {
                    rtype: ListenerTypes::On,
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
    /// event_listener.once("test", |_, _| {
    ///    println!("test");
    /// });
    /// ```
    pub fn once<T: Debug + 'static>(&mut self, name: &str, callback: ListenerCallback) {
        if self.listeners.iter().find(|x| x.name == name).is_none() {
            self.listeners.push(Event {
                name: name.to_string(),
                data: vec![],
            });
        }
        if self.max_listeners == 0
            || self
                .listeners
                .iter()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .len()
                < self.max_listeners
        {
            self.listeners
                .iter_mut()
                .find(|x| x.name == name)
                .unwrap()
                .data
                .push(Listener {
                    rtype: ListenerTypes::Once,
                    callback,
                });
        } else {
            panic!("Max listeners reached");
        }
    }

    /*
    /// Get existing events
    /// ## Returns
    /// [`Vec<&Event>`]
    pub fn get_events(&self) -> Vec<&Event> {
        self.listeners.iter().map(|x| x).collect::<Vec<_>>()
    }
    */

    /*
    /// Get existing event names
    /// ## Returns
    /// [`Vec<String>`]
    pub fn get_event_names(&self) -> Vec<String> {
        self.listeners.iter().map(|x| x.name.clone()).collect()
    }
    */

    /*
    /// Get all existent listeners of event
    /// ## Parameters
    /// * `name` - The name of the event
    /// ## Returns
    /// [`Vec<&Listener>`]
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut event_listener = EventListener::new();
    /// event_listener.on("test", |_, _| {
    ///   println!("test");
    /// });
    /// event_listener.get_listeners("test").iter().for_each(|x| {
    ///  println!("{:?}", x);
    /// });
    /// ```
    pub fn get_listeners(&self, name: &str) -> Vec<&Listener> {
        self.listeners
            .iter()
            .find(|x| x.name == name)
            .unwrap()
            .data
            .iter()
            .map(|x| x)
            .collect::<Vec<_>>()
    }
    */

    /// Remove all listeners of event
    /// ## Parameters
    /// * `name` - The name of the event
    /// ## Example
    /// ```
    /// use rust_event_listener::EventListener;
    /// let mut event_listener = EventListener::new();
    /// event_listener.on("test", |_, _| {
    ///  println!("test");
    /// });
    /// event_listener.remove_all_listeners("test");
    /// ```
    /// ## Returns
    /// [`bool`] - `true` if the event was removed, `false` if it wasn't
    pub fn remove_all_listeners(&mut self, name: &str) -> bool {
        if self.listeners.iter().find(|x| x.name == name).is_none() {
            return false;
        }
        self.listeners
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
    /// event_listener.on("test", |_, _| {
    /// println!("test");
    /// });
    /// event_listener.emit("test", "test");
    /// ```
    /// ## Panics
    /// If the event doesn't exist
    pub fn emit<T: Debug + 'static>(&mut self, name: &str, data: T) {
        if self.listeners.iter().find(|x| x.name == name).is_none() {
            panic!("Event doesn't exist");
        }
        let data_clone = Arc::new(data);
        for i in &self.listeners {
            if i.name == name {
                for j in &i.data {
                    (j.callback)(name.to_string(), Box::new(data_clone.clone()));
                }
            }
        }
    }
}
