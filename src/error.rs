use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationError {
    MissingLayoutData,
    InvalidLayoutData,
    OutputTooLong,
    InvalidUtf16,
    OsStatus(i32),
}

impl fmt::Display for TranslationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLayoutData => {
                formatter.write_str("the input source has no keyboard layout data")
            }
            Self::InvalidLayoutData => {
                formatter.write_str("the input source has invalid keyboard layout data")
            }
            Self::OutputTooLong => {
                formatter.write_str("the key translation exceeds the UTF-16 output limit")
            }
            Self::InvalidUtf16 => {
                formatter.write_str("the key translation contains invalid UTF-16")
            }
            Self::OsStatus(status) => {
                write!(formatter, "UCKeyTranslate returned OSStatus {status}")
            }
        }
    }
}

impl std::error::Error for TranslationError {}
