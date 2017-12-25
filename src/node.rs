

use std::time::SystemTime;
use std::sync::atomic::{AtomicPtr, Ordering};


pub struct Node {
    /* which means unique string */
    _name: AtomicPtr<String>,

    /* format(ipv4:port) */
    _address: AtomicPtr<String>,
    //prevLogIndex      uint64
    _last_update_time: AtomicPtr<SystemTime>,
}

impl Node {

    pub fn new() -> Node {
        let name = AtomicPtr::new(&mut "".to_owned());
        let addr = AtomicPtr::new(&mut "".to_owned());
        let time = AtomicPtr::new(&mut SystemTime::now());

        Node {
            _name: name,
            _address: addr,
            _last_update_time: time,
        }
    }
}