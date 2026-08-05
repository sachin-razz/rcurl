#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SshEngine;

#[allow(dead_code)]
impl SshEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn format_ssh_auth_request(user: &str) -> String {
        format!("SSH_MSG_USERAUTH_REQUEST user: {}, service: ssh-connection, method: publickey", user)
    }
}
