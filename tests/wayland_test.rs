/// Testes unitarios para deteccao de sessao Wayland.
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

#[test]
fn test_session_type_debug() {
    let session = SessionType::X11;
    assert_eq!(format!("{:?}", session), "X11");
}

#[test]
fn test_session_type_copy() {
    let original = SessionType::Wayland;
    let copied = original;
    assert_eq!(original, copied);
}

#[test]
fn test_session_type_all_variants_as_str() {
    let variants = [
        (SessionType::X11, "X11"),
        (SessionType::Wayland, "Wayland"),
        (SessionType::Unknown, "Unknown"),
    ];
    for (session, expected) in variants {
        assert_eq!(session.as_str(), expected);
    }
}

// "A liberdade e o direito de fazer tudo o que as leis permitem." - Montesquieu
