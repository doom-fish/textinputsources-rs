use core::ffi::{c_char, c_void, CStr};
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

#[derive(Debug)]
pub struct InputSource {
    pub(crate) handle: NonNull<c_void>,
    _not_send_sync: PhantomData<Rc<()>>,
}

impl InputSource {
    #[must_use]
    pub fn current_keyboard_layout() -> Option<Self> {
        let handle =
            ffi::with_tis_lock(|| unsafe { ffi::textinputsources_current_keyboard_layout() });
        NonNull::new(handle).map(|handle| Self {
            handle,
            _not_send_sync: PhantomData,
        })
    }

    #[must_use]
    pub fn current_keyboard_input_source() -> Option<Self> {
        let handle =
            ffi::with_tis_lock(|| unsafe { ffi::textinputsources_current_keyboard_input_source() });
        NonNull::new(handle).map(|handle| Self {
            handle,
            _not_send_sync: PhantomData,
        })
    }

    #[must_use]
    pub fn current_ascii_capable_keyboard_layout() -> Option<Self> {
        let handle = ffi::with_tis_lock(|| unsafe {
            ffi::textinputsources_current_ascii_capable_keyboard_layout()
        });
        NonNull::new(handle).map(|handle| Self {
            handle,
            _not_send_sync: PhantomData,
        })
    }

    #[must_use]
    pub fn source_id(&self) -> Option<String> {
        self.string_property(0)
    }

    #[must_use]
    pub fn localized_name(&self) -> Option<String> {
        self.string_property(1)
    }

    #[must_use]
    pub fn source_type(&self) -> Option<String> {
        self.string_property(2)
    }

    #[must_use]
    pub fn category(&self) -> Option<String> {
        self.string_property(3)
    }

    #[must_use]
    pub fn is_ascii_capable(&self) -> bool {
        ffi::with_tis_lock(|| unsafe {
            ffi::textinputsources_source_is_ascii_capable(self.handle.as_ptr())
        })
    }

    #[must_use]
    pub fn is_keyboard_input_mode(&self) -> bool {
        ffi::with_tis_lock(|| unsafe {
            ffi::textinputsources_source_is_keyboard_input_mode(self.handle.as_ptr())
        })
    }

    #[must_use]
    pub fn unicode_key_layout_data(&self) -> Option<Vec<u8>> {
        ffi::with_tis_lock(|| {
            let length =
                unsafe { ffi::textinputsources_source_layout_data_length(self.handle.as_ptr()) };
            let length = usize::try_from(length).ok()?;
            if length == 0 || length > 16 * 1024 * 1024 {
                return None;
            }
            let ffi_length = isize::try_from(length).ok()?;
            let mut data = vec![0; length];
            let copied = unsafe {
                ffi::textinputsources_source_copy_layout_data(
                    self.handle.as_ptr(),
                    data.as_mut_ptr(),
                    ffi_length,
                )
            };
            copied.then_some(data)
        })
    }

    fn string_property(&self, property: u32) -> Option<String> {
        let string = ffi::with_tis_lock(|| unsafe {
            ffi::textinputsources_source_string(self.handle.as_ptr(), property)
        });
        let string = NonNull::<c_char>::new(string)?;
        let value = unsafe { CStr::from_ptr(string.as_ptr()) }
            .to_str()
            .ok()
            .map(str::to_owned);
        unsafe { ffi::textinputsources_string_free(string.as_ptr()) };
        value
    }
}

impl Drop for InputSource {
    fn drop(&mut self) {
        ffi::with_tis_lock(|| unsafe {
            ffi::textinputsources_source_release(self.handle.as_ptr());
        });
    }
}
