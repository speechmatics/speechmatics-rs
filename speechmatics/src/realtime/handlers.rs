use anyhow::Result;
use log::{debug, error};
use serde_json::from_slice;

use crate::realtime::models;

// The Attach trait was my way to get the macro for attaching event handlers working in a hacky way.
// We should try to found a way to do it that doesn't require the user to import this trait if possible.
pub trait Attach {
    fn attach(&self, handlers: &mut EventHandlers);
}

pub type ErrorCallback = fn(models::Error) -> ();
impl Attach for ErrorCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_error = Some(*self);
    }
}
pub type InfoCallback = fn(models::Info) -> ();
impl Attach for InfoCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_info = Some(*self);
    }
}
pub type WarningCallback = fn(models::Warning) -> ();
impl Attach for WarningCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_warning = Some(*self);
    }
}
pub type RecognitionStartedCallback = fn(models::RecognitionStarted) -> ();
impl Attach for RecognitionStartedCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_recognition_started = Some(*self);
    }
}
pub type AddTranscriptCallback = fn(models::AddTranscript) -> ();
impl Attach for AddTranscriptCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_transcript = Some(*self);
    }
}
pub type AddPartialTranscriptCallback = fn(models::AddPartialTranscript) -> ();
impl Attach for AddPartialTranscriptCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_partial_transcript = Some(*self);
    }
}
pub type AddTranslationCallback = fn(models::AddTranslation) -> ();
impl Attach for AddTranslationCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_translation = Some(*self);
    }
}
pub type AddPartialTranslationCallback = fn(models::AddPartialTranslation) -> ();
impl Attach for AddPartialTranslationCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_partial_translation = Some(*self);
    }
}
pub type AudioAddedCallback = fn(models::AudioAdded) -> ();
impl Attach for AudioAddedCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_audio_added = Some(*self);
    }
}

pub struct EventHandlers {
    pub handle_error: Option<fn(models::Error) -> ()>,
    pub handle_info: Option<fn(models::Info) -> ()>,
    pub handle_warning: Option<fn(models::Warning) -> ()>,
    pub handle_recognition_started: Option<fn(models::RecognitionStarted) -> ()>,
    pub handle_add_transcript: Option<fn(models::AddTranscript) -> ()>,
    pub handle_add_partial_transcript: Option<fn(models::AddPartialTranscript) -> ()>,
    pub handle_add_translation: Option<fn(models::AddTranslation) -> ()>,
    pub handle_add_partial_translation: Option<fn(models::AddPartialTranslation) -> ()>,
    pub handle_audio_added: Option<fn(models::AudioAdded) -> ()>,
}

impl EventHandlers {
    pub fn new() -> Self {
        Self {
            handle_error: None,
            handle_info: None,
            handle_warning: None,
            handle_recognition_started: None,
            handle_add_transcript: None,
            handle_add_partial_transcript: None,
            handle_add_translation: None,
            handle_add_partial_translation: None,
            handle_audio_added: None,
        }
    }

    pub fn handle_event(&mut self, event: models::Messages, data: Vec<u8>) -> Result<()> {
        match event {
            models::Messages::Error => {
                if let Some(handle_error) = &self.handle_error {
                    let message: models::Error = from_slice(&data)?;
                    println!("{:?}", message);
                    handle_error(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::Info => {
                if let Some(handle_info) = &self.handle_info {
                    let message: models::Info = from_slice(&data)?;
                    handle_info(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::Warning => {
                if let Some(handle_warning) = &self.handle_warning {
                    let message: models::Warning = from_slice(&data)?;
                    handle_warning(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::RecognitionStarted => {
                if let Some(handle_recognition_started) = &self.handle_recognition_started {
                    let message: models::RecognitionStarted = from_slice(&data)?;
                    handle_recognition_started(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::AddTranscript => {
                if let Some(handle_add_transcript) = &self.handle_add_transcript {
                    let message: models::AddTranscript = from_slice(&data)?;
                    handle_add_transcript(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::AddPartialTranscript => {
                if let Some(handle_add_partial_transcript) = &self.handle_add_partial_transcript {
                    let message: models::AddPartialTranscript = from_slice(&data)?;
                    handle_add_partial_transcript(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::AddTranslation => {
                if let Some(handle_add_translation) = &self.handle_add_translation {
                    let message: models::AddTranslation = from_slice(&data)?;
                    handle_add_translation(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::AddPartialTranslation => {
                if let Some(handle_add_partial_translation) = &self.handle_add_partial_translation {
                    let message: models::AddPartialTranslation = from_slice(&data)?;
                    handle_add_partial_translation(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::AudioAdded => {
                if let Some(handle_audio_added) = &self.handle_audio_added {
                    let message: models::AudioAdded = from_slice(&data)?;
                    handle_audio_added(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            models::Messages::EndOfTranscript => {
                error!("Should not reach this line of code as EndOfTranscript signals the end of the control loop");
                Err(tungstenite::Error::AlreadyClosed.into())
            }
        }
    }
}
