use std::collections::HashMap;

pub struct Signal<T> {
    subscribers: HashMap<usize, Box<dyn Fn(&T)>>,
    next_id: usize,
}

impl<T> Signal<T> {
    pub fn new() -> Self {
        Signal {
            subscribers: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn subscribe(&mut self, callback: impl Fn(&T) + 'static) -> usize {
        let id = self.next_id;
        self.subscribers.insert(id, Box::new(callback));
        self.next_id += 1;
        id
    }

    pub fn call(&self, value: &T) {
        for callback in self.subscribers.values() {
            callback(value);
        }
    }

    pub fn unsubscribe(&mut self, id: usize) {
        self.subscribers.remove(&id);
    }

    pub fn cleanup(&mut self) {
        self.subscribers.clear();
        self.next_id = 0;
    }
}
