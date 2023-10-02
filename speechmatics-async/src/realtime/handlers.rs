
use anyhow::Result;
use futures::Future;
use log::debug;
use serde_json::from_slice;
use std::pin::Pin;

use crate::realtime::models;

pub trait Attach {
    fn attach(&self, handlers: &mut EventHandlers);
}

pub type ErrorHandler = fn(models::Error) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for ErrorHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_error = Some(*self);
    }
}
pub type InfoHandler = fn(models::Info) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for InfoHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_info = Some(*self);
    }
}
pub type WarningHandler = fn(models::Warning) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for WarningHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_warning = Some(*self);
    }
}
pub type RecognitionStartedHandler = fn(models::RecognitionStarted) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for RecognitionStartedHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_recognition_started = Some(*self);
    }
}
pub type AddTranscriptHandler = fn(models::AddTranscript) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for AddTranscriptHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_transcript = Some(*self);
    }
}
pub type AddPartialTranscriptHandler = fn(models::AddPartialTranscript) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for AddPartialTranscriptHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_partial_transcript = Some(*self);
    }
}
pub type AddTranslationHandler = fn(models::AddTranslation) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for AddTranslationHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_translation = Some(*self);
    }
}
pub type AddPartialTranslationHandler = fn(models::AddPartialTranslation) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for AddPartialTranslationHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_partial_translation = Some(*self);
    }
}
pub type AudioAddedHandler = fn(models::AudioAdded) -> Pin<Box<dyn Future<Output = ()>>>;
impl Attach for AudioAddedHandler {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_audio_added = Some(*self);
    }
}

#[derive(Clone, Copy)]
pub struct EventHandlers {
    handle_error: Option<fn(models::Error) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_info: Option<fn(models::Info) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_warning: Option<fn(models::Warning) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_recognition_started:
        Option<fn(models::RecognitionStarted) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_add_transcript: Option<fn(models::AddTranscript) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_add_partial_transcript:
        Option<fn(models::AddPartialTranscript) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_add_translation: Option<fn(models::AddTranslation) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_add_partial_translation:
        Option<fn(models::AddPartialTranslation) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_audio_added: Option<fn(models::AudioAdded) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
    handle_end_of_transcript:
        Option<fn(models::EndOfTranscript) -> Pin<Box<(dyn futures::Future<Output = ()> + 'static)>>>,
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
            handle_end_of_transcript: None,
        }
    }

    pub async fn handle_event(&mut self, event: models::Messages, data: Vec<u8>) -> Result<()> {
        match event {
            super::models::Messages::Error => {
                if let Some(handle_error) = &self.handle_error {
                    let message: models::Error = from_slice(&data)?;
                    handle_error(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::Info => {
                if let Some(handle_info) = &self.handle_info {
                    let message: models::Info = from_slice(&data)?;
                    handle_info(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::Warning => {
                if let Some(handle_warning) = &self.handle_warning {
                    let message: models::Warning = from_slice(&data)?;
                    handle_warning(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::RecognitionStarted => {
                if let Some(handle_recognition_started) = &self.handle_recognition_started {
                    let message: models::RecognitionStarted = from_slice(&data)?;
                    handle_recognition_started(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddTranscript => {
                if let Some(handle_add_transcript) = &self.handle_add_transcript {
                    let message: models::AddTranscript = from_slice(&data)?;
                    handle_add_transcript(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddPartialTranscript => {
                if let Some(handle_add_partial_transcript) = &self.handle_add_partial_transcript {
                    let message: models::AddPartialTranscript = from_slice(&data)?;
                    handle_add_partial_transcript(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddTranslation => {
                if let Some(handle_add_translation) = &self.handle_add_translation {
                    let message: models::AddTranslation = from_slice(&data)?;
                    handle_add_translation(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddPartialTranslation => {
                if let Some(handle_add_partial_translation) = &self.handle_add_partial_translation {
                    let message: models::AddPartialTranslation = from_slice(&data)?;
                    handle_add_partial_translation(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AudioAdded => {
                if let Some(handle_audio_added) = &self.handle_audio_added {
                    let message: models::AudioAdded = from_slice(&data)?;
                    handle_audio_added(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::EndOfTranscript => {
                if let Some(handle_end_of_transcript) = &self.handle_end_of_transcript {
                    let message: models::EndOfTranscript = from_slice(&data)?;
                    handle_end_of_transcript(message).await;
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
        }
    }
}
