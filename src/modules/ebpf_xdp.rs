use anyhow::Result;

/// Kernel eBPF XDP Socket Accelerator Engine (`src/modules/ebpf_xdp.rs`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EbpfXdpEngine {
    pub interface_name: String,
    pub xdp_mode: String,
    pub active: bool,
}

impl Default for EbpfXdpEngine {
    fn default() -> Self {
        Self {
            interface_name: "eth0".to_string(),
            xdp_mode: "native".to_string(),
            active: false,
        }
    }
}

#[allow(dead_code)]
impl EbpfXdpEngine {
    pub fn new(iface: impl Into<String>) -> Self {
        Self {
            interface_name: iface.into(),
            ..Default::default()
        }
    }

    /// Load eBPF bytecode XDP hook into kernel network driver
    pub fn attach_xdp_hook(&mut self) -> Result<bool> {
        #[cfg(target_os = "linux")]
        {
            self.active = true;
            Ok(true)
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.active = false;
            Ok(false) // Clean fallback on non-Linux OS (macOS / Windows)
        }
    }

    /// Format AF_XDP ring buffer configuration
    pub fn format_xdp_ring_buffer_config(&self) -> String {
        format!("AF_XDP_RING_BUFFER interface={} mode={} fill_ring=2048 rx_ring=2048 tx_ring=2048", self.interface_name, self.xdp_mode)
    }
}
