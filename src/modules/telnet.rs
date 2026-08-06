//! Telnet Network Terminal Command & IAC Option Negotiator (RFC 854 & RFC 855)

#[allow(dead_code)]
pub const IAC: u8 = 0xFF;  // Interpret As Command
#[allow(dead_code)]
pub const DONT: u8 = 0xFE;
#[allow(dead_code)]
pub const DO: u8 = 0xFD;
#[allow(dead_code)]
pub const WONT: u8 = 0xFC;
#[allow(dead_code)]
pub const WILL: u8 = 0xFB;
#[allow(dead_code)]
pub const SB: u8 = 0xFA;   // Subnegotiation Begin
#[allow(dead_code)]
pub const SE: u8 = 0xF0;   // Subnegotiation End

#[allow(dead_code)]
pub const OPT_ECHO: u8 = 0x01;
#[allow(dead_code)]
pub const OPT_SUPPRESS_GO_AHEAD: u8 = 0x03;
#[allow(dead_code)]
pub const OPT_TERMINAL_TYPE: u8 = 0x18;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct TelnetProtocolEngine;

#[allow(dead_code)]
impl TelnetProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn build_do(option: u8) -> [u8; 3] {
        [IAC, DO, option]
    }

    pub fn build_dont(option: u8) -> [u8; 3] {
        [IAC, DONT, option]
    }

    pub fn build_will(option: u8) -> [u8; 3] {
        [IAC, WILL, option]
    }

    pub fn build_wont(option: u8) -> [u8; 3] {
        [IAC, WONT, option]
    }

    pub fn build_terminal_type_subnegotiation(term_type: &str) -> Vec<u8> {
        let mut bytes = vec![IAC, SB, OPT_TERMINAL_TYPE, 0x00];
        bytes.extend_from_slice(term_type.as_bytes());
        bytes.push(IAC);
        bytes.push(SE);
        bytes
    }
}
