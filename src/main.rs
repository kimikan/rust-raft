
mod config;
mod node;
mod define;

use std::time::SystemTime;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicPtr, Ordering};

use std::collections::HashMap;

struct Message {
    _id:u32,
}

struct Server {
    _peers: RwLock<HashMap<String, Arc<node::Node>>>,
    _node: node::Node,
    _role: define::Role,

    //offset in peers
    _voted_for: String,
    _term: i64,

    //for log replication
    _commit_index: i64,
    _next_index: i64,

    _match_index: i64,
}

impl Server {
    fn new() -> Server {
        Server {
            _peers: Default::default(),
            _node: node::Node::new(),
            _role: define::Role::Empty,
            _voted_for: Default::default(),
            _term: 0,
            _commit_index: 0,
            _next_index: 0,
            _match_index: 0,
        }
    }

    fn event_loop(&self, msg:&Message) {
        match self._role {
            define::Role::Empty => {}
            define::Role::Candidate => {
                self.candidate_event_loop(msg);
            },
            define::Role::Master => {
                self.master_event_loop(msg);
            },
            define::Role::Follower => {
                self.follower_event_loop(msg);
            },
        }
    }

    fn candidate_event_loop(&self, msg:&Message) {}

    fn master_event_loop(&self, msg:&Message) {}

    fn follower_event_loop(&self, msg:&Message) {}
}

impl Server {}


fn main() {}
