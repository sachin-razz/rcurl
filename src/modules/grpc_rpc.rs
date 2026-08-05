use anyhow::Result;

/// gRPC & gRPC-Web Binary Protobuf Engine (`grpc://` / `grpcs://`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GrpcEngine {
    pub endpoint: String,
    pub service: String,
    pub method: String,
}

#[allow(dead_code)]
impl GrpcEngine {
    pub fn new(endpoint: impl Into<String>, service: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            service: service.into(),
            method: method.into(),
        }
    }

    /// Build gRPC HTTP/2 path header (e.g. `/package.Service/Method`)
    pub fn build_grpc_path(&self) -> String {
        format!("/{}/{}", self.service, self.method)
    }

    /// Format gRPC binary payload framing (1-byte flag + 4-byte big-endian length prefix)
    pub fn format_grpc_payload(data: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(5 + data.len());
        buf.push(0); // Compressed flag (0 = uncompressed)
        buf.extend_from_slice(&(data.len() as u32).to_be_bytes());
        buf.extend_from_slice(data);
        buf
    }
}

/// JSON-RPC 2.0 Method Invocation Engine (`--json-rpc`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JsonRpcEngine {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: u64,
}

#[allow(dead_code)]
impl JsonRpcEngine {
    pub fn new(method: impl Into<String>, params: Vec<serde_json::Value>, id: u64) -> Self {
        Self {
            method: method.into(),
            params,
            id,
        }
    }

    pub fn format_request_body(&self) -> Result<String> {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": self.method,
            "params": self.params,
            "id": self.id
        });
        Ok(serde_json::to_string(&payload)?)
    }
}

/// XML-RPC Method Invocation Engine (`--xml-rpc`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct XmlRpcEngine {
    pub method_name: String,
}

#[allow(dead_code)]
impl XmlRpcEngine {
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            method_name: method.into(),
        }
    }

    pub fn format_xml_payload(&self, params: &[&str]) -> String {
        let mut xml = format!("<?xml version=\"1.0\"?><methodCall><methodName>{}</methodName><params>", self.method_name);
        for p in params {
            xml.push_str(&format!("<param><value><string>{}</string></value></param>", p));
        }
        xml.push_str("</params></methodCall>");
        xml
    }
}
