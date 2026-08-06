//! DNS-over-HTTPS (DoH) Wireformat Query Engine (RFC 8484)

pub struct DohResolver;

impl DohResolver {

    /// Build binary DNS Header & Question section for A (type 1) or AAAA (type 28) lookup
    pub fn build_dns_query_wireformat(domain: &str, qtype: u16) -> Vec<u8> {
        let mut query = Vec::with_capacity(32 + domain.len());

        // 12-byte DNS Header: Dynamic Transaction ID, Standard Query Flags (0x0100)
        let tx_id = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos() & 0xFFFF) as u16;
        let [t1, t2] = tx_id.to_be_bytes();
        query.extend_from_slice(&[t1, t2, 0x01, 0x00]);
        // Questions: 1, Answer RRs: 0, Authority RRs: 0, Additional RRs: 0
        query.extend_from_slice(&[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // QNAME: Length-prefixed domain labels (e.g. 7"example"3"com"0)
        for part in domain.split('.') {
            let bytes = part.as_bytes();
            query.push(bytes.len() as u8);
            query.extend_from_slice(bytes);
        }
        query.push(0x00); // End of QNAME

        // QTYPE & QCLASS (IN = 1)
        query.extend_from_slice(&qtype.to_be_bytes());
        query.extend_from_slice(&1u16.to_be_bytes());

        query
    }

    /// Build URL-safe Base64 String for GET DNS-over-HTTPS endpoint (`?dns=<base64url>`)
    pub fn base64url_encode(input: &[u8]) -> String {
        let std_b64 = crate::modules::vauth::basic::base64_encode(input);
        std_b64.replace('+', "-").replace('/', "_").replace('=', "")
    }

    /// Format complete DoH GET URL (`https://cloudflare-dns.com/dns-query?dns=...`)
    pub fn build_doh_get_url(endpoint: &str, domain: &str) -> String {
        let wireformat = Self::build_dns_query_wireformat(domain, 1);
        let b64 = Self::base64url_encode(&wireformat);
        format!("{}?dns={}", endpoint, b64)
    }
}
