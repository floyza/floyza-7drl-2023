pub struct MessageLog {
    pub log: Vec<String>,
    pub current_messages: Vec<String>,
}

impl MessageLog {
    pub fn enqueue_message(&mut self, msg: &str) {
        self.log.push(msg.to_string());
        self.current_messages.push(msg.to_string());
    }
    pub fn clear_current(&mut self) {
        self.current_messages.clear();
    }
}
