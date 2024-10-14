use std::sync::Arc;

use crate::broker::Broker;

pub struct Producer {
    broker: Arc<Broker>,
}

impl Producer {
    pub fn new(broker: Arc<Broker>) -> Self {
        Self { broker }
    }

    pub fn produce(&self, topic: String, message: String) {
        self.broker.send(topic, message);
    }
}
