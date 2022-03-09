use std::fmt::Debug;

pub type ListenerCallback = Box<dyn Fn(String, Box<dyn Debug + 'static>)>;

#[derive(Debug)]
pub enum ListenerTypes {
    On,
    Once,
}

pub struct Listener {
    pub rtype: ListenerTypes,
    pub callback: ListenerCallback,
}

impl Debug for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Listener {{ rtype: {:?}, callback: f' }}", self.rtype)
    }
}
