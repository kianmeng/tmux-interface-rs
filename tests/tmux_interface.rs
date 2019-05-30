extern crate tmux_interface;
use crate::tmux_interface::TmuxInterface;


//#[test]
//fn attach_session() {
    //unimplemented!();
//}


//#[test]
//fn detach_client() {
    //unimplemented!();
//}


#[test]
fn has_session() {
    use crate::tmux_interface::NewSession;
    use std::borrow::Cow;

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(Cow::Borrowed("test_has_session")),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();
    assert_eq!(tmux.has_session("test_has_session").unwrap(), true);
    tmux.kill_session("test_has_session", false, false).unwrap();
}


//#[test]
//fn kill_server() {
    //unimplemented!();
//}


#[test]
fn kill_session() {
    use crate::tmux_interface::NewSession;
    use std::borrow::Cow;

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(Cow::Borrowed("test_kill_session")),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();
    tmux.kill_session("test_kill_session", false, false).unwrap();
}


//#[test]
//fn list_clients() {
    //unimplemented!();
//}


//#[test]
//fn list_commands() {
    //unimplemented!();
//}


#[test]
fn list_sessions() {
    use crate::tmux_interface::NewSession;
    use std::borrow::Cow;

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(Cow::Borrowed("test_list_sessions")),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();
    tmux.list_sessions(None).unwrap();
    tmux.kill_session("test_list_sessions", false, false).unwrap();
}


//#[test]
//fn lock_client() {
    //unimplemented!();
//}


//#[test]
//fn lock_session() {
    //unimplemented!();
//}


#[test]
fn new_session() {
    use crate::tmux_interface::NewSession;
    use std::borrow::Cow;

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(Cow::Borrowed("test_new_session")),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();
    tmux.kill_session("test_new_session", false, false).unwrap();
}


//#[test]
//fn refresh_client() {
    //unimplemented!();
//}


#[test]
fn rename_session() {
    use crate::tmux_interface::NewSession;
    use std::borrow::Cow;

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(Cow::Borrowed("test_rename_session")),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();
    tmux.rename_session(Some("test_rename_session"), "rename_test_session").unwrap();
    assert_eq!(tmux.has_session("rename_test_session").unwrap(), true);
    tmux.kill_session("rename_test_session", false, false).unwrap();
}


//#[test]
//fn show_messages() {
    //unimplemented!();
//}


//#[test]
//fn source_file() {
    //unimplemented!();
//}


//#[test]
//fn start_server() {
    //unimplemented!();
//}


//#[test]
//fn suspend_client() {
    //unimplemented!();
//}


//#[test]
//fn switch_client() {
    //unimplemented!();
//}





//#[test]
//fn list_sessions() {
    //use crate::session::Sessions;
    //use crate::tmux_interface::TmuxInterface;
    //use crate::LIST_SESSIONS_FORMAT;

    //let tmux = TmuxInterface::new();
    //let sessions_str = tmux.list_sessions(Some(LIST_SESSIONS_FORMAT)).unwrap();
    //let sessions = Sessions::parse(&sessions_str).unwrap();
    //for session in &sessions {
        //if session.id == 0 {
        //}
    //}
    //assert_eq!(sessions[0].id, 0);
//}