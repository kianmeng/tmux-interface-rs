#[test]
fn show_environment() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux show-environment [-gs] [-t target-session] [variable]
        // (alias: showenv)
        // ```
        //
        // tmux ^1.7:
        // ```text
        // tmux show-environment [-g] [-t target-session] [variable]
        // (alias: showenv)
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux show-environment [-g] [-t target-session]
        // (alias: showenv)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-environment", "-g", "-s", "-t", "1", "2"]"#
        );
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("1").to_string();
    tmux.show_environment(Some(true), Some(true), Some(&target_session), Some("2"))
        .unwrap_err();
}