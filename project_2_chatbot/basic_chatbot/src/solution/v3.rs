use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
<<<<<<< HEAD
            // Make sure you initialize your struct members here
            model,
            sessions: HashMap::new(),
=======
            model,
            chat_sessions: HashMap::new(),

>>>>>>> 7ae468e98b219be61752eb95f90c6ffc9fa3769b
        };
    }
}

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        if !self.sessions.contains_key(&username) {
            let new_chat = self.model.chat();
            self.sessions.insert(username.clone(), new_chat);
        }

        let chat = self.sessions.get_mut(&username).unwrap();

        match chat(&message).await {
            Ok(text) => text,
            Err(_) => String::from("Error"),
        }
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
