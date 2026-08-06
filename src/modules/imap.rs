//! IMAP4rev1 Protocol Command Formatter & Tagged Response Engine (RFC 3501)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ImapProtocolEngine {
    pub tag_counter: u32,
}

#[allow(dead_code)]
impl ImapProtocolEngine {
    pub fn new() -> Self {
        Self { tag_counter: 1 }
    }

    pub fn next_tag(&mut self) -> String {
        let tag = format!("A{:04}", self.tag_counter);
        self.tag_counter += 1;
        tag
    }

    pub fn format_login(&mut self, user: &str, pass: &str) -> String {
        let tag = self.next_tag();
        format!("{} LOGIN \"{}\" \"{}\"\r\n", tag, user, pass)
    }

    pub fn format_select(&mut self, mailbox: &str) -> String {
        let tag = self.next_tag();
        format!("{} SELECT \"{}\"\r\n", tag, mailbox)
    }

    pub fn format_fetch_headers(&mut self, sequence_set: &str) -> String {
        let tag = self.next_tag();
        format!("{} FETCH {} (FLAGS BODY[HEADER.FIELDS (DATE FROM SUBJECT)])\r\n", tag, sequence_set)
    }

    pub fn format_logout(&mut self) -> String {
        let tag = self.next_tag();
        format!("{} LOGOUT\r\n", tag)
    }
}
