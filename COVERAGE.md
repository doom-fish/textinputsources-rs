# COVERAGE

- Frameworks: Carbon/HIToolbox, Unicode Utilities, and Foundation.
- Scope: input source properties, keyboard translation, and keyboard selection change notifications.
- Legend: ✅ implemented, ⏭️ not implemented.

## Text Input Sources

| Apple API | Status | Rust API | Notes |
| --- | --- | --- | --- |
| `TISCopyCurrentKeyboardLayoutInputSource` | ✅ | `InputSource::current_keyboard_layout` | Owns and releases the copy result. |
| `TISCopyCurrentKeyboardInputSource` | ✅ | `InputSource::current_keyboard_input_source` | Owns and releases the copy result. |
| `TISCopyCurrentASCIICapableKeyboardLayoutInputSource` | ✅ | `InputSource::current_ascii_capable_keyboard_layout` | Owns and releases the copy result. |
| `TISGetInputSourceProperty`, `kTISPropertyInputSourceID` | ✅ | `InputSource::source_id` | Copies a borrowed string. |
| `kTISPropertyLocalizedName` | ✅ | `InputSource::localized_name` | Copies a borrowed string. |
| `kTISPropertyInputSourceType` | ✅ | `InputSource::source_type`, `is_keyboard_input_mode` | Compares the type with `kTISTypeKeyboardInputMode`. |
| `kTISPropertyInputSourceCategory` | ✅ | `InputSource::category` | Copies a borrowed string. |
| `kTISPropertyInputSourceIsASCIICapable` | ✅ | `InputSource::is_ascii_capable` | Returns `false` if the property is absent. |
| `kTISPropertyUnicodeKeyLayoutData` | ✅ | `InputSource::unicode_key_layout_data` | Copies bytes. The source keeps the borrowed property. |
| `TISCreateInputSourceList`, `TISSelectInputSource` | ⏭️ | | The crate does not change input sources. |

## Keyboard translation

| Apple API | Status | Rust API | Notes |
| --- | --- | --- | --- |
| `LMGetKbdType` | ✅ | `keyboard_type` | Returns the current keyboard type. |
| `UCKeyTranslate` | ✅ | `translate` | Uses the key-down action and a 64-unit UTF-16 buffer. |
| `kUCKeyTranslateNoDeadKeysMask` | ✅ | `TranslationOptions::NO_DEAD_KEYS` | `NONE` keeps dead keys active. |
| Dead-key state | ✅ | `DeadKeyState` | State passes between calls; a Space call can complete a dead key. |
| Other `UCKeyTranslate` key actions | ⏭️ | | The crate translates key-down events only. |

## Notifications

| Apple API | Status | Rust API | Notes |
| --- | --- | --- | --- |
| `NSTextInputContextKeyboardSelectionDidChangeNotification` | ✅ | `KeyboardSelectionObserver` | Observes the default notification center. |
| `NSNotificationCenter.removeObserver` | ✅ | `KeyboardSelectionObserver` drop | Deactivates the callback before removal. |
