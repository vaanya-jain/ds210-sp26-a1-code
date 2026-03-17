use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chat_sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model,
            chat_sessions: HashMap::new(),

        };
    }
}

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        return String::from("Hello, I am not a bot (yet)!");
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
         if let Some(chat) = self.sessions.get(&username) {
        let history = chat
            .session()
            .unwrap()   
            .history(); 

        history
            .into_iter()
            .map(|msg| msg.content().to_string())
            .collect()
    } else {
        Vec::new()
    }
}
