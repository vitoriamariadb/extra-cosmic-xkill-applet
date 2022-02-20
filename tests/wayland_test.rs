use extra_cosmic_xkill::wayland::SessionType;

#[test]
fn test_session_type_as_str() {
    assert_eq!(SessionType::X11.as_str(), "X11");
    assert_eq!(SessionType::Wayland.as_str(), "Wayland");
    assert_eq!(SessionType::Unknown.as_str(), "Unknown");
}

#[test]
fn test_session_type_equality() {
    assert_eq!(SessionType::X11, SessionType::X11);
    assert_eq!(SessionType::Wayland, SessionType::Wayland);
    assert_ne!(SessionType::X11, SessionType::Wayland);
}

#[test]
fn test_session_type_clone() {
    let session = SessionType::X11;
    let cloned = session.clone();
    assert_eq!(session, cloned);
}
