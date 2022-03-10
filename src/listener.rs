use core::fmt::Debug;

pub type ListenerCallback = Box<dyn Fn(String, String)>;

#[derive(Debug)]
pub enum ListenerTypes {
    On,
    Once,
}

pub struct Listener {
    pub rtype: ListenerTypes,
    pub callback: ListenerCallback,
}