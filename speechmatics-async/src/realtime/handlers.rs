//! This module is auto-generated - do not edit!
use anyhow::Result;
use futures::Future;
use log::debug;
use serde_json::from_slice;

use crate::realtime::models;

pub trait Attach {
    fn attach(&self, handlers: &mut EventHandlers);
}

pub type ErrorCallback = fn(models::Error) -> dyn Future<Output = ()>;
impl Attach for ErrorCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_error = Some(*self);
    }
}
pub type InfoCallback = fn(models::Info) -> dyn Future<Output = ()>;
impl Attach for InfoCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_info = Some(*self);
    }
}
pub type WarningCallback = fn(models::Warning) -> dyn Future<Output = ()>;
impl Attach for WarningCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_warning = Some(*self);
    }
}
pub type RecognitionStartedCallback = fn(models::RecognitionStarted) -> dyn Future<Output = ()>;
impl Attach for RecognitionStartedCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_recognition_started = Some(*self);
    }
}
pub type AddTranscriptCallback = fn(models::AddTranscript) -> dyn Future<Output = ()>;
impl Attach for AddTranscriptCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_transcript = Some(*self);
    }
}
pub type AddPartialTranscriptCallback = fn(models::AddPartialTranscript) -> dyn Future<Output = ()>;
impl Attach for AddPartialTranscriptCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_partial_transcript = Some(*self);
    }
}
pub type AddTranslationCallback = fn(models::AddTranslation) -> dyn Future<Output = ()>;
impl Attach for AddTranslationCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_translation = Some(*self);
    }
}
pub type AddPartialTranslationCallback = fn(models::AddPartialTranslation) -> dyn Future<Output = ()>;
impl Attach for AddPartialTranslationCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_add_partial_translation = Some(*self);
    }
}
pub type AudioAddedCallback = fn(models::AudioAdded) -> dyn Future<Output = ()>;
impl Attach for AudioAddedCallback {
    fn attach(&self, handlers: &mut EventHandlers) {
        handlers.handle_audio_added = Some(*self);
    }
}

pub struct EventHandlers {
    handle_error: Option<fn(models::Error) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_info: Option<fn(models::Info) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_warning: Option<fn(models::Warning) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_recognition_started:
        Option<fn(models::RecognitionStarted) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_add_transcript: Option<fn(models::AddTranscript) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_add_partial_transcript:
        Option<fn(models::AddPartialTranscript) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_add_translation: Option<fn(models::AddTranslation) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_add_partial_translation:
        Option<fn(models::AddPartialTranslation) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_audio_added: Option<fn(models::AudioAdded) -> (dyn futures::Future<Output = ()> + 'static)>,
    handle_end_of_transcript:
        Option<fn(models::EndOfTranscript) -> (dyn futures::Future<Output = ()> + 'static)>,
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
                    handle_error(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::Info => {
                if let Some(handle_info) = &self.handle_info {
                    let message: models::Info = from_slice(&data)?;
                    handle_info(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::Warning => {
                if let Some(handle_warning) = &self.handle_warning {
                    let message: models::Warning = from_slice(&data)?;
                    handle_warning(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::RecognitionStarted => {
                if let Some(handle_recognition_started) = &self.handle_recognition_started {
                    let message: models::RecognitionStarted = from_slice(&data)?;
                    handle_recognition_started(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddTranscript => {
                if let Some(handle_add_transcript) = &self.handle_add_transcript {
                    let message: models::AddTranscript = from_slice(&data)?;
                    handle_add_transcript(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddPartialTranscript => {
                if let Some(handle_add_partial_transcript) = &self.handle_add_partial_transcript {
                    let message: models::AddPartialTranscript = from_slice(&data)?;
                    handle_add_partial_transcript(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddTranslation => {
                if let Some(handle_add_translation) = &self.handle_add_translation {
                    let message: models::AddTranslation = from_slice(&data)?;
                    handle_add_translation(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AddPartialTranslation => {
                if let Some(handle_add_partial_translation) = &self.handle_add_partial_translation {
                    let message: models::AddPartialTranslation = from_slice(&data)?;
                    handle_add_partial_translation(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::AudioAdded => {
                if let Some(handle_audio_added) = &self.handle_audio_added {
                    let message: models::AudioAdded = from_slice(&data)?;
                    handle_audio_added(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
            super::models::Messages::EndOfTranscript => {
                if let Some(handle_end_of_transcript) = &self.handle_end_of_transcript {
                    let message: models::EndOfTranscript = from_slice(&data)?;
                    handle_end_of_transcript(message);
                    Ok(())
                } else {
                    debug!("No handler attached for event");
                    Ok(())
                }
            }
        }
    }
}
