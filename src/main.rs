use std::{sync::Arc, thread, time::Duration};

use pop::{broker, consumer, producer};

fn main() {
    let broker = Arc::new(broker::Broker::new());
    broker.create_topic("test".to_string());

    let producer = producer::Producer::new(broker.clone());
    let consumer = consumer::Consumer::new(broker.clone(), "test".to_string());

    thread::spawn(move || {
        consumer.consume();
    });

    for i in 0..10 {
        producer.produce("test".to_string(), format!("message-{}", i % 3));
        thread::sleep(Duration::from_secs(2));
    }
}
