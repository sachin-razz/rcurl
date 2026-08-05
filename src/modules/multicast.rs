use std::net::{IpAddr, SocketAddr};

/// Omni-Multicast Dual-Stack IPv4/IPv6 Engine (`src/modules/multicast.rs`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OmniMulticastEngine {
    pub ipv4_group: Option<SocketAddr>,
    pub ipv6_group: Option<SocketAddr>,
    pub ssm_source: Option<IpAddr>,
    pub pgm_fec_repair: bool,
}

impl Default for OmniMulticastEngine {
    fn default() -> Self {
        Self {
            ipv4_group: "239.255.0.1:9999".parse().ok(),
            ipv6_group: "[ff05::1]:9999".parse().ok(),
            ssm_source: None,
            pgm_fec_repair: true,
        }
    }
}

#[allow(dead_code)]
impl OmniMulticastEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ssm_source(mut self, source_ip: IpAddr) -> Self {
        self.ssm_source = Some(source_ip);
        self
    }

    pub fn format_igmpv3_join_group(&self) -> String {
        match self.ssm_source {
            Some(src) => format!("IGMPv3_SSM_JOIN group=239.255.0.1 source={}", src),
            None => "IGMPv3_ASM_JOIN group=239.255.0.1".to_string(),
        }
    }

    pub fn format_pgm_nak_repair(&self, sequence_num: u64) -> Vec<u8> {
        let mut buf = Vec::with_capacity(16);
        buf.extend_from_slice(b"PGM_NAK_REPAIR_");
        buf.extend_from_slice(&sequence_num.to_be_bytes());
        buf
    }
}
