use std::collections::HashMap;

pub mod rest_source;
pub mod paxos;

pub trait SourceOps {
    fn fetch(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub(crate) struct Sources<T> where T: SourceOps {
    sources: HashMap<String, T>,
}

impl<T> Sources<T> where T: SourceOps {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn register(&mut self, source: T, source_name: &str) {
        self.sources.insert(source_name.to_string(), source);
    }

    pub fn load(&self, source_name: &str) -> Option<&T> {
        self.sources.get(source_name)
    }
}
