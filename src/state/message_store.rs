use std::sync::{Arc, RwLock};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}

pub struct MessageStore {
    messages: RwLock<VecDeque<Message>>,
    max_size: usize,
}

pub type SharedMessageStore = Arc<MessageStore>;

impl MessageStore {
    pub fn new(max_size: usize) -> SharedMessageStore {
        Arc::new(Self {
            messages: RwLock::new(VecDeque::new()),
            max_size,
        })
    }

    pub fn add(&self, message: Message) {
        let mut messages = self.messages.write().unwrap();
        messages.push_back(message);
        while messages.len() > self.max_size {
            messages.pop_front();
        }
    }

    pub fn get_all(&self) -> Vec<Message> {
        self.messages.read().unwrap().clone().into()
    }

    pub fn get_recent(&self, count: usize) -> Vec<Message> {
        let messages = self.messages.read().unwrap();
        let start = messages.len().saturating_sub(count);
        messages.range(start..).cloned().collect()
    }

    pub fn clear(&self) {
        self.messages.write().unwrap().clear();
    }

    pub fn len(&self) -> usize {
        self.messages.read().unwrap().len()
    }
}
