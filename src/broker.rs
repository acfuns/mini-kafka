use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

type TopicName = String;
type Message = String;

pub struct Broker {
    topics: Arc<Mutex<HashMap<TopicName, VecDeque<Message>>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            topics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 创建一个新的主题
    pub fn create_topic(&self, topic_name: String) {
        let mut topics = self.topics.lock().unwrap();
        topics.insert(topic_name, VecDeque::new());
    }

    // 发布消息到主题
    pub fn send(&self, topic: String, message: String) {
        let mut topics = self.topics.lock().unwrap();
        if let Some(queue) = topics.get_mut(&topic) {
            queue.push_back(message);
        } else {
            // todo: 这里应该返回一个错误
            println!("Topic not found: {}", topic)
        }
    }

    // 订阅主题
    pub fn receive(&self, topic: String) -> Option<Message> {
        let mut topics = self.topics.lock().unwrap();
        topics.get_mut(&topic)?.pop_front()
    }
}
