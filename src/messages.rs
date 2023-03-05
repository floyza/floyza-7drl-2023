use crate::State;

pub struct MessageLog {
    pub log: Vec<String>,
    pub queue: Vec<String>,
}

impl MessageLog {
    pub fn enqueue_message(&mut self, msg: &str) {
        self.queue.push(msg.to_string());
    }
}

pub fn handle_messages(state: &mut State) {
    for msg in std::mem::take(&mut state.messages.queue) {
        println!("{}", msg);
        state.messages.log.push(msg);
    }
}
