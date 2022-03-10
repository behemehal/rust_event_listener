use core::fmt::Debug;
/// EventListener callback closure
pub type ListenerCallback = Box<dyn Fn(String, String)>;

#[derive(Debug)]
/// Listener types
pub enum ListenerTypes {
    /// Listener that will be called on emit
    On,
    /// Listener that will be called only once
    Once,
}

/// Listener struct
pub struct Listener {
    /// Listener type
    pub rtype: ListenerTypes,
    /// Callback function
    pub callback: ListenerCallback,
}

impl Debug for Listener {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Listener {{ rtype: {:?}, callback: f' }}", self.rtype)
    }
}
