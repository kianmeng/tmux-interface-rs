#![allow(non_upper_case_globals)]

extern crate regex;


use std::process::{Command, Output};
use std::str;
use regex::Regex;
use super::tmux_interface_error::TmuxInterfaceError;


pub const _2_KEY: &str = "-2";

// constants for use as keys for subcommands
pub const a_KEY: &str = "-a";
pub const b_KEY: &str = "-b";
pub const c_KEY: &str = "-c";
pub const d_KEY: &str = "-d";
pub const e_KEY: &str = "-e";
pub const f_KEY: &str = "-f";
pub const g_KEY: &str = "-g";
pub const h_KEY: &str = "-h";
//pub const i_KEY: &str = "-i";
//pub const j_KEY: &str = "-j";
pub const k_KEY: &str = "-k";
pub const l_KEY: &str = "-l";
pub const m_KEY: &str = "-m";
pub const n_KEY: &str = "-n";
//pub const o_KEY: &str = "-o";
pub const p_KEY: &str = "-p";
//pub const Q_KEY: &str = "-q";
pub const r_KEY: &str = "-r";
pub const s_KEY: &str = "-s";
pub const t_KEY: &str = "-t";
pub const u_KEY: &str = "-u";
pub const v_KEY: &str = "-v";
//pub const w_KEY: &str = "-w";
pub const x_KEY: &str = "-x";
pub const y_KEY: &str = "-y";
//pub const z_KEY: &str = "-z";


#[allow(non_upper_case_globals)]
pub const A_KEY: &str = "-A";
//pub const B_KEY: &str = "-B";
pub const C_KEY: &str = "-C";
pub const D_KEY: &str = "-D";
pub const E_KEY: &str = "-E";
pub const F_KEY: &str = "-F";
//pub const G_KEY: &str = "-G";
//pub const H_KEY: &str = "-H";
//pub const I_KEY: &str = "-I";
//pub const J_KEY: &str = "-J";
//pub const K_KEY: &str = "-K";
pub const L_KEY: &str = "-L";
pub const M_KEY: &str = "-M";
pub const N_KEY: &str = "-N";
//pub const O_KEY: &str = "-O";
pub const P_KEY: &str = "-P";
//pub const Q_KEY: &str = "-Q";
pub const R_KEY: &str = "-R";
pub const S_KEY: &str = "-S";
pub const T_KEY: &str = "-T";
pub const U_KEY: &str = "-U";
pub const V_KEY: &str = "-V";
//pub const W_KEY: &str = "-W";
pub const X_KEY: &str = "-X";
//pub const Y_KEY: &str = "-Y";
//pub const Z_KEY: &str = "-Z";


pub const CC_KEY: &str = "-C";


// XXX: mb also add_env, clear_env, remove_env for std::process::Command?
pub struct TmuxInterface<'a> {
    /// Tmux binary name (default: `tmux`, can be set as `tmux_mock.sh` for "sniffing")
    pub tmux: Option<&'a str>,                          // tmux (or tmux_mock.sh)
    pub colours256: Option<bool>,                       // -2
    pub control_mode: Option<bool>,                     // -C
    pub disable_echo: Option<bool>,                     // -CC
    pub shell_cmd: Option<&'a str>,                     // -c shell-command
    pub file: Option<&'a str>,                          // -f file
    pub socket_name: Option<&'a str>,                   // -L socket-name
    pub login_shell: Option<bool>,                      // -l
    pub socket_path: Option<&'a str>,                   // -S socket-path
    pub force_utf8: Option<bool>,                       // -u
    pub verbose_logging: Option<bool>,                  // -v
    //pub version                                       // -V
}


impl<'a> Default for TmuxInterface<'a> {
    fn default() -> Self {
        TmuxInterface {
            tmux: None,
            colours256: None,
            control_mode: None,
            disable_echo: None,
            shell_cmd: None,
            file: None,
            socket_name: None,
            login_shell: None,
            socket_path: None,
            force_utf8: None,
            verbose_logging: None
            //version
        }
    }
}


impl<'a> TmuxInterface<'a> {

    const TMUX: &'static str = "tmux";
    const VERSION_STR_REGEX: &'static str = r"^tmux\s(\d+).(\d+)\n$";


    pub fn new() -> Self {
        Default::default()
    }

    /// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path]
    /// [command [flags]]
    pub fn subcommand(&self, subcmd: &str, args: &[&str]) -> Result<Output, TmuxInterfaceError> {
        let mut options: Vec<&str> = Vec::new();
        let mut tmux = Command::new(self.tmux.unwrap_or(TmuxInterface::TMUX));
        if self.colours256.unwrap_or(false) { options.push(_2_KEY); };
        if self.control_mode.unwrap_or(false) { options.push(C_KEY); };
        if self.disable_echo.unwrap_or(false) { options.push(CC_KEY); };
        if self.login_shell.unwrap_or(false) { options.push(l_KEY) };
        if self.force_utf8.unwrap_or(false) { options.push(u_KEY) }
        if self.verbose_logging.unwrap_or(false) { options.push(v_KEY) }
        self.shell_cmd.as_ref().and_then(|s| Some(options.extend_from_slice(&[c_KEY, &s])));
        self.file.as_ref().and_then(|s| Some(options.extend_from_slice(&[f_KEY, &s])));
        self.socket_name.as_ref().and_then(|s| Some(options.extend_from_slice(&[L_KEY, &s])));
        self.socket_path.as_ref().and_then(|s| Some(options.extend_from_slice(&[S_KEY, &s])));
        tmux.args(options);
        tmux.arg(subcmd);
        let output = tmux.args(args).output()?;
        Ok(output)
    }


    //pub fn exec(&self, subcmd: &str, args: &[String]) -> Result<ExitStatus, MyError> {
        //let mut tmux = Command::new(self.tmux);
        //tmux.arg(subcmd);
        //let status = tmux.args(args).status()?;
        ////println!("{} {} {:?}", Tmux::TMUX, subcmd, args);
        ////println!("[exec output]: {} {} {:?} {:?}", self.tmux, subcmd, args, status);
        ////if !status.success() {
            ////println!("[exec status error]: {}", status);
        ////}
        //Ok(status)
    //}


    // tmux parameter
    // ===========================================================================================

    /// ```text
    /// tmux -V
    /// ```
    pub fn version(&self) -> Result<(usize, usize), TmuxInterfaceError> {
        let mut tmux = Command::new(self.tmux.unwrap_or(TmuxInterface::TMUX));
        let output = tmux.arg(V_KEY).output()?;
        let version_str = String::from_utf8_lossy(&output.stdout).to_string();
        let regex = Regex::new(TmuxInterface::VERSION_STR_REGEX)?;
        if let Some(cap) = regex.captures(&version_str) {
            let major = cap[1].parse::<usize>()?;
            let minor = cap[2].parse::<usize>()?;
            Ok((major, minor))
        } else {
            Err(TmuxInterfaceError::new("cap"))
        }
    }
}