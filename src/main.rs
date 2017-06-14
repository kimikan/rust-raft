
use std::time::SystemTime;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicPtr, Ordering};

use std::collections::HashMap;

struct Configuration {
    _nodes: Vec<String>,
}

impl Configuration {
    fn new() -> Configuration {
        Configuration { _nodes: vec![] }
    }

    fn get_nodes(&self) -> &Vec<String> {
        &self._nodes
    }
}


struct Node {
    /* which means unique string */
    _name: AtomicPtr<String>,

    /* format(ipv4:port) */
    _address: AtomicPtr<String>,
    //prevLogIndex      uint64
    _last_update_time: AtomicPtr<SystemTime>,
}

impl Node {

    fn new() -> Node {
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

enum Role {
    /*only 1 master in a cluster */
    Master,

    /* temp state for lead vote */
    Candidate,
    /* a more general role */
    Follower,
    Empty,
}

struct Message {
    _id:u32,
}

struct Server {
    _peers: RwLock<HashMap<String, Arc<Node>>>,
    _node: Node,
    _role: Role,

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
            _peers: HashMap::new(),
            _node: Node::new(),
            _role: Role::Empty,
            _voted_for: 0,
            _term: 0,
            _commit_index: 0,
            _next_index: 0,
            _match_index: 0,
        }
    }

    fn event_loop(&self, msg:&Message) {
        match self._role {
            Role::Empty => {}
            Role::Candidate => {
                self.candidate_event_loop(msg);
            },
            Role::Master => {
                self.master_event_loop(msg);
            },
            Role::Follower => {
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
