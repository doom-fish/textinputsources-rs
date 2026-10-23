# textinputsources-rs

Safe Rust bindings for macOS Text Input Sources and Unicode keyboard layout translation. The Swift bridge uses Carbon/HIToolbox, Unicode Utilities, and Foundation. The crate requires macOS 12 or later and Rust 1.82 or later.

Set `MACOSX_DEPLOYMENT_TARGET=12.0` when you build an application that uses this crate. Cargo does not set an application's deployment target from the Swift package.

## Quick start

```rust
use textinputsources::{
    keyboard_type, translate, DeadKeyState, InputSource, ModifierState, TranslationOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layout = InputSource::current_keyboard_layout().expect("no keyboard layout");
    let mut dead_key_state = DeadKeyState::default();
    let text = translate(
        &layout,
        0,
        ModifierState::NONE,
        keyboard_type(),
        &mut dead_key_state,
        TranslationOptions::NONE,
    )?;
    println!("{}: {text}", layout.localized_name().unwrap_or_default());
    Ok(())
}
```

## Input sources

`InputSource::current_keyboard_layout` gets the layout for keyboard translation. `current_keyboard_input_source` gets the selected input source, which can be an input mode. `current_ascii_capable_keyboard_layout` gets an ASCII-capable layout.

Each `InputSource` owns its TIS copy result and releases it on drop. The methods copy the source ID, localized name, type, category, and Unicode layout data. A missing property gives `None`. Layout data larger than 16 MiB also gives `None`. `is_ascii_capable` and `is_keyboard_input_mode` give `false` when the property is missing.

## Keyboard translation

`keyboard_type` gets the current hardware keyboard type. `translate` uses the layout data of the retained `InputSource` and the key-down action.

`ModifierState` uses Carbon's modifier bits after a shift of eight bits. Combine `COMMAND` (1), `SHIFT` (2), `CAPS_LOCK` (4), `OPTION` (8), and `CONTROL` (16) with `|`. Use `from_bits` for an existing modifier value.

Pass a mutable `DeadKeyState` to each call. Use a new state when the layout changes. If a shortcut needs the dead-key glyph, translate key code 49 (Space) with the same modifiers when `is_active` is true. `TranslationOptions::NONE` keeps dead keys active. `NO_DEAD_KEYS` disables them. A translation returns at most 63 UTF-16 units, and an error leaves the supplied state unchanged.

## Keyboard selection changes

`KeyboardSelectionObserver::new` observes `NSTextInputContextKeyboardSelectionDidChangeNotification` on the default notification center. The callback runs on the thread that posts the notification. It must implement `Fn() + Send + Sync + 'static`. Drop the observer to remove its registration. An active callback can finish after the observer drops.

## Example

```bash
cargo run --example 01_current_keyboard_layout
```

## License

MIT OR Apache-2.0
