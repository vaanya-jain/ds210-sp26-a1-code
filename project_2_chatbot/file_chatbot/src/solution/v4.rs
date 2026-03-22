use crate::solution::file_library;
use kalosm::language::*;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 { model: model };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self
            .model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session
        let old_session = file_library::load_chat_session_from_file(filename);
        match old_session {
            Some(session) => chat_session = chat_session.with_session(session),
            None => {
            }
        }
        let response = match chat_session(&message).await {
            Ok(text) => text,
            Err(_) => return String::from("Error generating response"),
        };

        file_library::save_chat_session_to_file(filename,&chat_session.session().unwrap());

        return response;
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            }
            Some(session) => {
                let history = session.history();

                let mut messages: Vec<String> = Vec::new();

                for msg in history {
                    match msg.role() {
                        MessageType::SystemPrompt => {}
                        _ => messages.push(String::from(msg.content())),
                    }
                }

                return messages;
            }
        }
    }
}
