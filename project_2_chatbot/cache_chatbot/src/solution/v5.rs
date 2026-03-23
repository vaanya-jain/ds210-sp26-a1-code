use file_chatbot::solution::file_library;
use kalosm::language::*;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                let mut new_chat: Chat<Llama> = self
                    .model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                match file_library::load_chat_session_from_file(&filename) {
                    None => {}
                    Some(session) => {
                        new_chat = new_chat.with_session(session);
                    }
                }

                let response = match new_chat(&message).await {
                    Ok(text) => text,
                    Err(_) => return String::from("Error generating response"),
                };
                
                file_library::save_chat_session_to_file(&filename, &new_chat.session().unwrap());

                self.cache.insert_chat(username, new_chat);

                return response;
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");

                let response = match chat_session(&message).await {
                    Ok(text) => text,
                    Err(_) => return String::from("Error generating response"),
                };

                file_library::save_chat_session_to_file(&filename, &chat_session.session().unwrap());

                return response;
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");

                let mut chat_session: Chat<Llama> = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                if let Some(session) = file_library::load_chat_session_from_file(filename) {
                    chat_session = chat_session.with_session(session);
                }
                
                self.cache.insert_chat(username.clone(), chat_session);

                if let Some(chat_session) = self.cache.get_chat(&username) {
                    let mut messages: Vec<String> = Vec::new();
                    for msg in chat_session.session().unwrap().history() {
                        match msg.role() {
                            MessageType::SystemPrompt => {}
                            _ => messages.push(String::from(msg.content())),
                        }
                    }

                    return messages;
                }
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");

                let mut messages: Vec<String> = Vec::new();
                for msg in chat_session.session().unwrap().history() {
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
