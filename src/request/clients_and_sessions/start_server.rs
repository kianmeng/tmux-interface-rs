use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
impl<'a> TmuxInterface<'a> {
    const START_SERVER: &'static str = "start-server";

    /// Start the tmux server, if not already running, without creating any sessions
    ///
    /// # Manual
    ///
    /// ```text
    /// tmux start-server
    /// (alias: start)
    /// ```
    pub fn start_server(&mut self) -> Result<Output, Error> {
        let output = self.subcommand(TmuxInterface::START_SERVER, &[""])?;
        Ok(output)
    }
}