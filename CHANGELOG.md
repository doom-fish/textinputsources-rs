# Changelog

This file records the changes to `textinputsources-rs`.

## [Unreleased]

### Added

- `InputSource` owns the selected keyboard layout, input source, or ASCII-capable layout.
- Input source methods copy the ID, name, type, category, and Unicode layout data. They also report ASCII capability and keyboard input modes.
- `translate` uses a bounded UTF-16 result, explicit modifier options, and a caller-owned dead-key state.
- `keyboard_type` gets the hardware keyboard type.
- `KeyboardSelectionObserver` removes its notification registration on drop.
