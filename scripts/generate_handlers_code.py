output = "//! This module is auto-generated - do not edit!\n\n"
output += "pub struct EventHandlers {\n"

for message in messages_enum:
    output += f"\thandle_{message}: Option<Box<dyn Fn(super::models::{message})>>,\n"

output += "}\n\n"

output += "impl EventHandlers {\n"

output += "\tpub fn new() -> Self {\n"
output += "\t\tSelf {\n"
            handle_error: None,
            handle_add_transcript: None
        }
    }
    pub fn register_event_handler(&mut self, message_type: super::models::Messages, value: Box<dyn Fn()>) {
        match message_type {
            super::models::Messages::Error => self.handle_error = Some(value),
            super::models::Messages::StartRecognition => self.handle_error = Some(value),
        }
    }
}