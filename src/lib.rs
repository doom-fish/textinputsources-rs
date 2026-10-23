#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod ffi;
mod input_source;
mod key_translation;
mod observer;

pub use error::TranslationError;
pub use input_source::InputSource;
pub use key_translation::{
    keyboard_type, translate, DeadKeyState, ModifierState, TranslationOptions,
};
pub use observer::{KeyboardSelectionObserver, KEYBOARD_SELECTION_DID_CHANGE_NOTIFICATION};
