use super::error::Error;
use super::tmux_interface::*;
use std::process::Output;

/// Structure for open the command prompt in a client
///
/// # Manual
///
/// ```text
/// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
/// ```
#[derive(Default, Debug)]
pub struct CommandPrompt<'a> {
    /// [-1] makesthe prompt only accept one key press
    pub one_keypress: Option<bool>,
    /// [-i] execute the command every time the prompt input changes
    pub on_input_change: Option<bool>,
    /// [-I inputs] - comma-separated list of the initial text for each prompt
    pub inputs: Option<&'a str>,
    /// [-p prompts] - prompts is a comma-separated list of prompts which are displayed in order
    pub prompts: Option<&'a str>,
    /// [-t target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [template] - template
    pub template: Option<&'a str>,
}

impl<'a> CommandPrompt<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// ```text
/// tmux display-menu [-c target-client] [-t target-pane] [-T title]
/// [-x position] [-y position] name key command ...
/// ```
#[derive(Default, Debug)]
pub struct DisplayMenu<'a> {
    /// [-c target-client] - target-client
    pub target_client: Option<&'a str>,
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>,
    /// [-T title] - title
    pub title: Option<&'a str>,
    /// [-x position] - x position of the menu
    pub x: Option<usize>,
    /// [-y position] - y position of the menu
    pub y: Option<usize>,
    // name - name
    //pub name: &'a str,
    // key - key
    //pub key: &'a str,
    // command ... - command
    //pub command: &'a str,
}

impl<'a> DisplayMenu<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Structure for displaying a message
///
/// # Manual
///
/// ```text
/// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
///  (alias: display)
/// ```
#[derive(Default, Debug)]
pub struct DisplayMessage<'a> {
    /// [-a] - list the format variables and their values
    pub list_format_vars: Option<bool>,
    /// [-I] - forward any input read from stdin to the empty pane given by target-pane
    pub forward_stdin: Option<bool>,
    /// [-p] - the output is printed to stdout
    pub print: Option<bool>,
    /// [-v] - print verbose logging as the format is parsed
    pub verbose: Option<bool>, // [-v]
    /// [-c target-client] - target-client
    pub target_client: Option<&'a str>, // [-c target-client]
    /// [-t target-pane] - target-pane
    pub target_pane: Option<&'a str>, // [-t target-pane]
    /// [message] - message
    pub message: Option<&'a str>, // [message]
}

impl<'a> DisplayMessage<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// All functions from man tmux "Status line" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#STATUS_LINE)
impl<'a> TmuxInterface<'a> {
    const COMMAND_PROMPT: &'static str = "command-prompt";
    const CONFIRM_BEFORE: &'static str = "confirm-before";
    const DISPLAY_MENU: &'static str = "display-menu";
    const DISPLAY_MESSAGE: &'static str = "display-message";

    /// # Manual
    ///
    /// ```text
    /// tmux command-prompt [-1i] [-I inputs] [-p prompts] [-t target-client] [template]
    /// ```
    pub fn command_prompt(
        &mut self,
        command_prompt: Option<&CommandPrompt>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(command_prompt) = command_prompt {
            if command_prompt.one_keypress.unwrap_or(false) {
                args.push(_1_KEY);
            }
            if command_prompt.on_input_change.unwrap_or(false) {
                args.push(i_KEY);
            }
            if let Some(s) = command_prompt.inputs {
                args.extend_from_slice(&[I_KEY, &s])
            }
            if let Some(s) = command_prompt.prompts {
                args.extend_from_slice(&[p_KEY, &s])
            }
            if let Some(s) = command_prompt.target_client {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = command_prompt.template {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::COMMAND_PROMPT, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux confirm-before [-p prompt] [-t target-client] command
    /// (alias: confirm)
    /// ```
    pub fn confirm_before(
        &mut self,
        prompt: Option<&str>,
        target_client: Option<&str>,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(s) = prompt {
            args.extend_from_slice(&[p_KEY, &s])
        }
        if let Some(s) = target_client {
            args.extend_from_slice(&[t_KEY, &s])
        }
        args.push(command);
        let output = self.subcommand(TmuxInterface::CONFIRM_BEFORE, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux display-menu [-c target-client] [-t target-pane] [-T title]
    /// [-x position] [-y position] name key command ...
    /// (alias: menu)
    /// ```
    ///
    pub fn display_menu(
        &mut self,
        display_menu: Option<&DisplayMenu>,
        name: &str,
        key: &str,
        command: &str,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let x;
        let y;
        if let Some(display_menu) = display_menu {
            if let Some(s) = display_menu.target_client {
                args.extend_from_slice(&[c_KEY, &s])
            }
            if let Some(s) = display_menu.target_pane {
                args.extend_from_slice(&[t_KEY, &s])
            }
            if let Some(s) = display_menu.title {
                args.extend_from_slice(&[T_KEY, &s])
            }
            if let Some(position) = display_menu.x {
                x = position.to_string();
                args.extend_from_slice(&[x_KEY, &x]);
            }
            if let Some(position) = display_menu.y {
                y = position.to_string();
                args.extend_from_slice(&[y_KEY, &y]);
            }
        }
        args.push(&name);
        args.push(&key);
        args.push(&command);
        let output = self.subcommand(TmuxInterface::DISPLAY_MENU, &args)?;
        Ok(output)
    }

    /// # Manual
    ///
    /// ```text
    /// tmux display-message [-aIpv] [-c target-client] [-t target-pane] [message]
    /// (alias: display)
    /// ```
    pub fn display_message(
        &mut self,
        display_message: Option<&DisplayMessage>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(display_message) = display_message {
            if display_message.list_format_vars.unwrap_or(false) {
                args.push(a_KEY);
            }
            if display_message.forward_stdin.unwrap_or(false) {
                args.push(I_KEY);
            }
            if display_message.print.unwrap_or(false) {
                args.push(p_KEY);
            }
            if display_message.verbose.unwrap_or(false) {
                args.push(v_KEY);
            }
            if let Some(s) = display_message.target_client {
                args.extend_from_slice(&[c_KEY, s])
            }
            if let Some(s) = display_message.target_pane {
                args.extend_from_slice(&[t_KEY, s])
            }
            if let Some(s) = display_message.message {
                args.push(&s)
            }
        }
        let output = self.subcommand(TmuxInterface::DISPLAY_MESSAGE, &args)?;
        Ok(output)
    }
}
