use std::time::Duration;
use regex::Regex;
use crate::TmuxInterfaceError;


pub const SESSION_VARS_SEPARATOR: &str = ":";
// XXX: mb make all fields optional
// FIXME: regex name can be anything, and other keys should be checked better
// NOTE: no colons or periods (ref: int session_check_name(const char *name))
pub const SESSION_VARS_REGEX_VEC: [(&str, &str); 15] = [
    ("session_alerts",          r"(\w+)?"),
    ("session_attached",        r"(\d+)?"),
    ("session_activity",        r"(\d+)?"),
    ("session_created",         r"(\d+)?"),
    ("session_format",          r"(\w+)?"),
    ("session_last_attached",   r"(\d+)?"),
    ("session_group",           r"(\w+)?"),
    ("session_group_size",      r"(\w+)?"),
    ("session_group_list",      r"(\w+)?"),
    ("session_grouped",         r"(\w+)?"),
    ("session_id",              r"\$(\d+)?"),
    ("session_many_attached",   r"(\w+)?"),
    ("session_name",            r"(\w+)?"),
    ("session_stack",           r"([\w,]*)?"),
    ("session_windows",         r"(\d+)?"),
];


//struct asdf<'a> {
    //vec: Vec<(&'a str, &'a str)>,
    //separator: &'a str
//}


//impl<'a> asdf<'a> {
    //fn new(vec: &Vec<(&str, &str)>, separator: &str) {
    //}


    //fn get_format(&self) -> String {
        //self.vec.iter()
            //.map(|t| format!("#{{{}}}", t.0))
            //.collect::<Vec<String>>()
            //.join(self.separator)
    //}


    //fn get_regex(&self) -> String {
        //format!("^{}$", self.vec.iter()
                //.map(|t| t.1)
                //.collect::<Vec<&str>>()
                //.join(self.separator)
        //)
    //}
//}


// accordingly to tmux.h: Formats
// XXX: check all types
#[derive(Default, Clone, Debug)]
pub struct Session {
    pub alerts: Option<String>,
    pub attached: Option<usize>,
    pub activity: Option<Duration>,
    pub created: Option<Duration>,
    pub format: Option<String>,
    pub last_attached: Option<Duration>,
    pub group: Option<String>,
    pub group_size: Option<String>,
    pub group_list: Option<String>,
    pub grouped: Option<String>,
    pub id: Option<usize>,
    pub many_attached: Option<String>,
    pub name: Option<String>,
    pub stack: Option<String>,
    pub windows: Option<usize>,
}


impl Session {

    pub fn new() -> Self {
        Default::default()
    }


    //pub fn parse2(session_str: &str) -> Result<Session, TmuxInterfaceError> {
        //let session_vars: Vec<&str> = session_str.split(SESSION_VARS_SEPARATOR).collect();
        //let mut session = Session::new();
        //session.alerts = session_vars[0].parse().ok();
        //session.attached = session_vars[1].parse().ok();
        //session.activity = session_vars[2].parse().ok().map(Duration::from_millis);
        //session.created = session_vars[3].parse().ok().map(Duration::from_millis);
        //session.format = session_vars[4].parse().ok();
        //session.last_attached = session_vars[5].parse().ok().map(Duration::from_millis);
        //session.group = session_vars[6].parse().ok();
        //session.group_size = session_vars[7].parse().ok();
        //session.group_list = session_vars[8].parse().ok();
        //session.grouped = session_vars[9].parse().ok();
        //session.id = session_vars[10].parse().ok();
        //session.many_attached = session_vars[11].parse().ok();
        //session.name = session_vars[12].parse().ok();
        //session.stack = session_vars[13].parse().ok();
        //session.windows = session_vars[14].parse().ok();
        //Ok(session)
    //}


    // XXX: mb deserialize?
    // XXX: mb callback
    pub fn parse(session_str: &str) -> Result<Session, TmuxInterfaceError> {
        let regex_str = format!("^{}$", SESSION_VARS_REGEX_VEC.iter()
                                .map(|t| t.1).collect::<Vec<&str>>().join(SESSION_VARS_SEPARATOR));
        let regex = Regex::new(&regex_str)?;
        let caps = regex.captures(session_str).unwrap();
        let mut session = Session::new();

        // XXX: optimize?
        session.alerts = caps.get(1).and_then(|s| s.as_str().parse().ok());
        session.attached = caps.get(2).and_then(|s| s.as_str().parse().ok());
        session.activity = caps.get(3)
            .and_then(|s| s.as_str().parse().ok())
            .and_then(|u| Some(Duration::from_millis(u)));
        session.created = caps.get(4)
            .and_then(|s| s.as_str().parse().ok())
            .and_then(|u| Some(Duration::from_millis(u)));
        session.format = caps.get(5).and_then(|s| s.as_str().parse().ok());
        session.last_attached = caps.get(6)
            .and_then(|s| s.as_str().parse().ok())
            .and_then(|u| Some(Duration::from_millis(u)));
        session.group = caps.get(7).and_then(|s| s.as_str().parse().ok());
        session.group_size = caps.get(8).and_then(|s| s.as_str().parse().ok());
        session.group_list = caps.get(9).and_then(|s| s.as_str().parse().ok());
        session.grouped = caps.get(10).and_then(|s| s.as_str().parse().ok());
        session.id = caps.get(11).and_then(|s| s.as_str().parse().ok());
        session.many_attached = caps.get(12).and_then(|s| s.as_str().parse().ok());
        session.name = caps.get(13).and_then(|s| s.as_str().parse().ok());
        session.stack = caps.get(14).and_then(|s| s.as_str().parse().ok());
        session.windows = caps.get(15).and_then(|s| s.as_str().parse().ok());
        Ok(session)
    }
}
