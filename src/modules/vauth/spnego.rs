//! SPNEGO / GSSAPI Kerberos Negotiate Token Builder (RFC 4178 & RFC 2743)

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SpnegoAuth;

#[allow(dead_code)]
impl SpnegoAuth {
    pub fn new() -> Self {
        Self
    }

    pub fn build_gssapi_spnego_token(inner_token: &[u8]) -> Vec<u8> {
        let mut token = Vec::with_capacity(16 + inner_token.len());
        token.push(0x60);
        let spnego_oid: &[u8] = &[0x06, 0x06, 0x2b, 0x06, 0x01, 0x05, 0x05, 0x02];

        let total_len = spnego_oid.len() + inner_token.len();
        token.push(total_len as u8);
        token.extend_from_slice(spnego_oid);
        token.extend_from_slice(inner_token);

        token
    }

    pub fn build_negotiate_header(kerberos_ticket: &[u8]) -> String {
        let spnego_token = Self::build_gssapi_spnego_token(kerberos_ticket);
        let b64 = crate::modules::vauth::basic::base64_encode(&spnego_token);
        format!("Negotiate {}", b64)
    }
}
