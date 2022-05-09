use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Move to the previous layout in the session
///
/// # Manual
///
/// tmux ^1.3:
/// ```text
/// tmux previous-layout [-t target-window]
/// (alias: prevl)
/// ```
#[derive(Debug, Default, Clone)]
pub struct PreviousLayout<'a> {
    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_3")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> PreviousLayout<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_1_3")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(PREVIOUS_LAYOUT);

        // `[-t target-window]`
        #[cfg(feature = "tmux_1_3")]
        if let Some(target_window) = &self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window.as_ref());
        }

        cmd
    }
}
