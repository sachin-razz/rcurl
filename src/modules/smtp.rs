#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SmtpProtocolEngine;

#[allow(dead_code)]
impl SmtpProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn build_ehlo_command(domain: &str) -> String {
        format!("EHLO {}\r\n", domain)
    }

    pub fn build_mail_from(sender: &str) -> String {
        format!("MAIL FROM:<{}>\r\n", sender)
    }
}
