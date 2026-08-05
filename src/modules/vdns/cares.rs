#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CaresDnsEngine;

#[allow(dead_code)]
impl CaresDnsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn format_cares_channel_option() -> &'static str {
        "ARES_OPT_FLAGS | ARES_FLAG_NOCHECKRESP"
    }
}
