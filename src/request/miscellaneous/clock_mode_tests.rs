#[test]
fn clock_mode() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux clock-mode [-t target-pane]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["clock-mode", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.clock_mode(Some("1")).unwrap_err();
}