use textinputsources::InputSource;

#[test]
fn current_layout_has_an_id_name_and_layout_data() {
    let source = InputSource::current_keyboard_layout().expect("a keyboard layout must exist");
    assert_ne!(source.source_id().expect("a layout must have an ID"), "");
    assert_ne!(
        source.localized_name().expect("a layout must have a name"),
        ""
    );
    assert!(source.source_type().is_some());
    assert!(source.category().is_some());
    assert_ne!(
        source
            .unicode_key_layout_data()
            .expect("a keyboard layout must have Unicode layout data"),
        Vec::<u8>::new()
    );
}

#[test]
fn selected_input_source_and_ascii_layout_are_available() {
    let source = InputSource::current_keyboard_input_source().expect("an input source must exist");
    assert_ne!(
        source.source_id().expect("an input source must have an ID"),
        ""
    );
    assert!(!source.is_keyboard_input_mode() || source.source_type().is_some());

    let ascii = InputSource::current_ascii_capable_keyboard_layout()
        .expect("an ASCII-capable layout must exist");
    assert!(ascii.is_ascii_capable());
    assert!(ascii.unicode_key_layout_data().is_some());
}
