use core::ffi::{c_char, c_void};
use std::sync::{Mutex, PoisonError};

static TIS_LOCK: Mutex<()> = Mutex::new(());

pub fn with_tis_lock<T>(operation: impl FnOnce() -> T) -> T {
    let _guard = TIS_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    operation()
}

extern "C" {
    pub fn textinputsources_current_keyboard_layout() -> *mut c_void;
    pub fn textinputsources_current_keyboard_input_source() -> *mut c_void;
    pub fn textinputsources_current_ascii_capable_keyboard_layout() -> *mut c_void;
    pub fn textinputsources_source_release(source: *mut c_void);
    pub fn textinputsources_source_string(source: *mut c_void, property: u32) -> *mut c_char;
    pub fn textinputsources_string_free(string: *mut c_char);
    pub fn textinputsources_source_is_ascii_capable(source: *mut c_void) -> bool;
    pub fn textinputsources_source_is_keyboard_input_mode(source: *mut c_void) -> bool;
    pub fn textinputsources_source_layout_data_length(source: *mut c_void) -> isize;
    pub fn textinputsources_source_copy_layout_data(
        source: *mut c_void,
        destination: *mut u8,
        length: isize,
    ) -> bool;
    pub fn textinputsources_keyboard_type() -> u32;
    pub fn textinputsources_translate(
        source: *mut c_void,
        key_code: u16,
        modifier_state: u32,
        keyboard_type: u32,
        options: u32,
        dead_key_state: *mut u32,
        output: *mut u16,
        capacity: isize,
        length: *mut isize,
    ) -> i32;
    pub fn textinputsources_observer_new(
        callback: unsafe extern "C" fn(*mut c_void),
        context: *mut c_void,
        retain: unsafe extern "C" fn(*mut c_void),
        release: unsafe extern "C" fn(*mut c_void),
    ) -> *mut c_void;
    pub fn textinputsources_observer_release(observer: *mut c_void);
}
