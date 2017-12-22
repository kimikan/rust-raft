

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
