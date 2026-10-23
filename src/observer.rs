use core::ffi::c_void;
use core::ptr::NonNull;

use doom_fish_utils::callback_context::CallbackContext;

use crate::ffi;

pub const KEYBOARD_SELECTION_DID_CHANGE_NOTIFICATION: &str =
    "NSTextInputContextKeyboardSelectionDidChangeNotification";

type KeyboardSelectionCallback = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Debug)]
pub struct KeyboardSelectionObserver {
    handle: NonNull<c_void>,
    context: CallbackContext<KeyboardSelectionCallback>,
}

unsafe extern "C" fn keyboard_selection_changed(context: *mut c_void) {
    let _ = unsafe {
        CallbackContext::<KeyboardSelectionCallback>::with(
            context,
            "textinputsources::keyboard_selection_changed",
            |callback| callback(),
        )
    };
}

impl KeyboardSelectionObserver {
    #[must_use]
    pub fn new(callback: impl Fn() + Send + Sync + 'static) -> Self {
        let callback: KeyboardSelectionCallback = Box::new(callback);
        let context = CallbackContext::new(callback);
        let handle = unsafe {
            ffi::textinputsources_observer_new(
                keyboard_selection_changed,
                context.as_ptr(),
                CallbackContext::<KeyboardSelectionCallback>::RETAIN,
                CallbackContext::<KeyboardSelectionCallback>::RELEASE,
            )
        };
        Self {
            handle: NonNull::new(handle)
                .expect("registering a notification observer returns a token"),
            context,
        }
    }
}

impl Drop for KeyboardSelectionObserver {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { ffi::textinputsources_observer_release(self.handle.as_ptr()) };
    }
}
