use textinputsources::{
    keyboard_type, translate, DeadKeyState, InputSource, ModifierState, TranslationOptions,
};

#[test]
fn current_or_ascii_layout_translates_a_plain_key() {
    for layout in [
        InputSource::current_keyboard_layout(),
        InputSource::current_ascii_capable_keyboard_layout(),
    ]
    .into_iter()
    .flatten()
    {
        let mut state = DeadKeyState::default();
        if let Ok(text) = translate(
            &layout,
            0,
            ModifierState::NONE,
            keyboard_type(),
            &mut state,
            TranslationOptions::NONE,
        ) {
            if !text.is_empty() {
                assert!(!state.is_active());
                return;
            }
        }
    }
    panic!("no keyboard layout translated the plain ANSI A key");
}

#[test]
fn command_shift_and_option_combinations_translate() {
    let layout = InputSource::current_ascii_capable_keyboard_layout()
        .expect("an ASCII-capable layout must exist");
    let keyboard_type = keyboard_type();
    for modifiers in [0, 1, 2, 8, 1 | 2, 1 | 8, 2 | 8, 1 | 2 | 8] {
        let mut state = DeadKeyState::default();
        translate(
            &layout,
            0,
            ModifierState::from_bits(modifiers),
            keyboard_type,
            &mut state,
            TranslationOptions::NONE,
        )
        .expect("a key must translate with each modifier combination");
    }
}

#[test]
fn dead_key_state_round_trips_when_a_layout_has_dead_keys() {
    let keyboard_type = keyboard_type();
    for layout in [
        InputSource::current_keyboard_layout(),
        InputSource::current_ascii_capable_keyboard_layout(),
    ]
    .into_iter()
    .flatten()
    {
        for modifiers in [
            ModifierState::OPTION,
            ModifierState::OPTION | ModifierState::SHIFT,
            ModifierState::NONE,
            ModifierState::SHIFT,
        ] {
            for key_code in 0..128 {
                let mut state = DeadKeyState::default();
                if translate(
                    &layout,
                    key_code,
                    modifiers,
                    keyboard_type,
                    &mut state,
                    TranslationOptions::NONE,
                )
                .is_err()
                    || !state.is_active()
                {
                    continue;
                }
                let mut resumed = state;
                if let Ok(text) = translate(
                    &layout,
                    49,
                    modifiers,
                    keyboard_type,
                    &mut resumed,
                    TranslationOptions::NONE,
                ) {
                    if !text.is_empty() && !resumed.is_active() {
                        let mut without_dead_keys = DeadKeyState::default();
                        translate(
                            &layout,
                            key_code,
                            modifiers,
                            keyboard_type,
                            &mut without_dead_keys,
                            TranslationOptions::NO_DEAD_KEYS,
                        )
                        .expect("the key must translate without dead keys");
                        assert!(!without_dead_keys.is_active());
                        assert!(state.is_active());
                        return;
                    }
                }
            }
        }
    }
    eprintln!("no available layout has a dead key with a Space completion");
}
