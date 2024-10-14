use std::{sync::Arc, thread, time::Duration};

use crate::broker::Broker;

pub struct Consumer {
    broker: Arc<Broker>,
    topic: String,
}

impl Consumer {
    pub fn new(broker: Arc<Broker>, topic: String) -> Self {
        Self { broker, topic }
    }

    pub fn consume(&self) {
        loop {
            if let Some(message) = self.broker.receive(self.topic.clone()) {
                println!("Consumer: message: {}", message);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}
