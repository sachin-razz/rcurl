//! POP3 Mail Protocol Client Session Engine (RFC 1939)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Pop3ProtocolEngine;

#[allow(dead_code)]
impl Pop3ProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn format_user(&self, username: &str) -> String {
        format!("USER {}\r\n", username)
    }

    pub fn format_pass(&self, password: &str) -> String {
        format!("PASS {}\r\n", password)
    }

    pub fn format_stat(&self) -> &'static str {
        "STAT\r\n"
    }

    pub fn format_list(&self, msg_num: Option<u32>) -> String {
        if let Some(n) = msg_num {
            format!("LIST {}\r\n", n)
        } else {
            "LIST\r\n".to_string()
        }
    }

    pub fn format_retr(&self, msg_num: u32) -> String {
        format!("RETR {}\r\n", msg_num)
    }

    pub fn format_dele(&self, msg_num: u32) -> String {
        format!("DELE {}\r\n", msg_num)
    }

    pub fn format_quit(&self) -> &'static str {
        "QUIT\r\n"
    }
}
