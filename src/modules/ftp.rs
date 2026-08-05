#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FtpProtocolEngine {
    pub passive_mode: bool,
}

#[allow(dead_code)]
impl FtpProtocolEngine {
    pub fn new(passive: bool) -> Self {
        Self { passive_mode: passive }
    }

    pub fn build_pwd_command(&self) -> &'static str {
        "PWD\r\n"
    }

    pub fn build_pasv_command(&self) -> &'static str {
        "PASV\r\n"
    }
}
