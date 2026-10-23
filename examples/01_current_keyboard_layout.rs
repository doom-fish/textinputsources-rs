use textinputsources::{
    keyboard_type, translate, DeadKeyState, InputSource, ModifierState, TranslationOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layout = InputSource::current_keyboard_layout().expect("no keyboard layout is available");
    println!(
        "{} ({})",
        layout.localized_name().unwrap_or_default(),
        layout.source_id().unwrap_or_default()
    );

    let mut dead_key_state = DeadKeyState::default();
    let text = translate(
        &layout,
        0,
        ModifierState::NONE,
        keyboard_type(),
        &mut dead_key_state,
        TranslationOptions::NONE,
    )?;
    println!("key code 0: {text}");
    Ok(())
}
