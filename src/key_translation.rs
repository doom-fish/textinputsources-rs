use core::ops::{BitOr, BitOrAssign};

use crate::{ffi, InputSource, TranslationError};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeadKeyState(u32);

impl DeadKeyState {
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn is_active(self) -> bool {
        self.0 != 0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModifierState(u32);

impl ModifierState {
    pub const NONE: Self = Self(0);
    pub const COMMAND: Self = Self(1);
    pub const SHIFT: Self = Self(2);
    pub const CAPS_LOCK: Self = Self(4);
    pub const OPTION: Self = Self(8);
    pub const CONTROL: Self = Self(16);

    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl BitOr for ModifierState {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for ModifierState {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TranslationOptions(u32);

impl TranslationOptions {
    pub const NONE: Self = Self(0);
    pub const NO_DEAD_KEYS: Self = Self(1);

    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

#[must_use]
pub fn keyboard_type() -> u32 {
    ffi::with_tis_lock(|| unsafe { ffi::textinputsources_keyboard_type() })
}

pub fn translate(
    layout: &InputSource,
    key_code: u16,
    modifier_state: ModifierState,
    keyboard_type: u32,
    dead_key_state: &mut DeadKeyState,
    options: TranslationOptions,
) -> Result<String, TranslationError> {
    const UTF16_CAPACITY: usize = 64;
    let mut output = [0_u16; UTF16_CAPACITY];
    let mut length = 0_isize;
    let mut next_dead_key_state = dead_key_state.0;
    let status = ffi::with_tis_lock(|| unsafe {
        ffi::textinputsources_translate(
            layout.handle.as_ptr(),
            key_code,
            modifier_state.bits(),
            keyboard_type,
            options.bits(),
            &raw mut next_dead_key_state,
            output.as_mut_ptr(),
            isize::try_from(UTF16_CAPACITY).expect("the output buffer fits in isize"),
            &raw mut length,
        )
    });
    match status {
        0 => {}
        1 => return Err(TranslationError::MissingLayoutData),
        2 => return Err(TranslationError::InvalidLayoutData),
        3 => return Err(TranslationError::OutputTooLong),
        status => return Err(TranslationError::OsStatus(status)),
    }
    let length = usize::try_from(length).map_err(|_| TranslationError::OutputTooLong)?;
    let string = String::from_utf16(
        output
            .get(..length)
            .ok_or(TranslationError::OutputTooLong)?,
    )
    .map_err(|_| TranslationError::InvalidUtf16)?;
    *dead_key_state = DeadKeyState(next_dead_key_state);
    Ok(string)
}
