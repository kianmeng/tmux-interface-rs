#[test]
fn list_panes() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux list-panes [-as] [-F format] [-t target]
        // (alias: lsp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["list-panes", "-a", "-s", "-F", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.list_panes(Some(true), Some(true), Some("1"), Some("2"))
        .unwrap_err();
}