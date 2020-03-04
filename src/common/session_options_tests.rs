#[test]
fn activity() {
    use crate::common::session_options::Activity;
    assert_eq!(Activity::On.to_string(), "on");
    assert_eq!(Activity::Off.to_string(), "off");
    assert_eq!(Activity::Both.to_string(), "both");
}

#[test]
fn action() {
    use crate::common::session_options::Action;
    assert_eq!(Action::Any.to_string(), "any");
    assert_eq!(Action::None.to_string(), "none");
    assert_eq!(Action::Current.to_string(), "current");
    assert_eq!(Action::Other.to_string(), "other");
}

#[test]
fn status() {
    use crate::common::session_options::Status;
    assert_eq!(Status::On.to_string(), "on");
    assert_eq!(Status::Off.to_string(), "off");
    assert_eq!(Status::_2.to_string(), "2");
    assert_eq!(Status::_3.to_string(), "3");
    assert_eq!(Status::_4.to_string(), "4");
    assert_eq!(Status::_5.to_string(), "5");
}

#[test]
fn status_justify() {
    use crate::common::session_options::StatusJustify;
    assert_eq!(StatusJustify::Left.to_string(), "left");
    assert_eq!(StatusJustify::Centre.to_string(), "centre");
    assert_eq!(StatusJustify::Right.to_string(), "right");
}

#[test]
fn status_position() {
    use crate::common::session_options::StatusPosition;
    assert_eq!(StatusPosition::Top.to_string(), "top");
    assert_eq!(StatusPosition::Bottom.to_string(), "bottom");
}

#[test]
fn parse() {
    use crate::SessionOptions;

    let session_options_str = r#"
        activity-action other
        assume-paste-time 1
        base-index 1
        bell-action none
        default-command ""
        default-shell "/usr/bin/fish"
        destroy-unattached off
        detach-on-destroy on
        display-panes-active-colour red
        display-panes-colour blue
        display-panes-time 1000
        display-time 750
        history-limit 2000
        key-table "root"
        lock-after-time 0
        lock-command "lock -np"
        message-command-style fg=blue,bg=black
        message-style fg=colour232,bg=colour166,bright
        mouse on
        prefix C-a
        prefix2 None
        renumber-windows off
        repeat-time 500
        set-titles on
        set-titles-string ""
        silence-action other
        status on
        status-interval 2
        status-justify left
        status-keys emacs
        status-left ""
        status-left-length 50
        status-left-style default
        status-position bottom
        status-right ""
        status-right-length 50
        status-right-style default
        status-style fg=colour247,bg=#282c34
        visual-activity off
        visual-bell off
        visual-silence off
        word-separators ""
    "#;
    let _session_options = session_options_str.parse::<SessionOptions>().unwrap();
    //dbg!(&session_options);
}

#[test]
fn to_string() {
    use crate::SessionOptionsBuilder;

    let _session_options = SessionOptionsBuilder::new()
        .default_shell("asdfasdfasdf")
        .build();
    //dbg!(&session_options.to_string());
}

#[test]
fn get() {
    use crate::SessionOptions;
    let _session_options = SessionOptions::get_all().unwrap();
    //dbg!(session_options);
}