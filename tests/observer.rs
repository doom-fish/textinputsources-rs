use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use foundation::NotificationCenter;
use textinputsources::{KeyboardSelectionObserver, KEYBOARD_SELECTION_DID_CHANGE_NOTIFICATION};

#[test]
fn observer_registers_and_removes_itself_on_drop() {
    let center = NotificationCenter::default_center();
    let hits = Arc::new(AtomicUsize::new(0));
    let weak_hits = Arc::downgrade(&hits);
    let observed_hits = Arc::clone(&hits);
    let observer = KeyboardSelectionObserver::new(move || {
        observed_hits.fetch_add(1, Ordering::SeqCst);
    });

    center
        .post(KEYBOARD_SELECTION_DID_CHANGE_NOTIFICATION)
        .unwrap();
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    drop(observer);
    center
        .post(KEYBOARD_SELECTION_DID_CHANGE_NOTIFICATION)
        .unwrap();
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    drop(hits);
    assert!(weak_hits.upgrade().is_none());
}
