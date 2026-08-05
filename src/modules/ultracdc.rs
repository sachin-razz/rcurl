use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// UltraCDC Normalized Chunk Definition with Merkle Root Hashing
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UltraCdcChunk {
    pub offset: u64,
    pub length: usize,
    pub chunk_hash: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UltraCdcEngine {
    pub min_size: usize,
    pub avg_size: usize,
    pub max_size: usize,
    pub primary_mask: u32,
    pub secondary_mask: u32,
}

impl Default for UltraCdcEngine {
    fn default() -> Self {
        Self {
            min_size: 2048,
            avg_size: 8192,
            max_size: 65536,
            primary_mask: 0x0003FFF0,   // UltraCDC normalized primary boundary mask
            secondary_mask: 0x00007FF0, // UltraCDC normalized secondary boundary mask
        }
    }
}

#[allow(dead_code)]
impl UltraCdcEngine {
    pub fn new(min_size: usize, avg_size: usize, max_size: usize) -> Self {
        Self {
            min_size,
            avg_size,
            max_size,
            primary_mask: 0x0003FFF0,
            secondary_mask: 0x00007FF0,
        }
    }

    /// Perform UltraCDC normalized dual-mask chunking on file data
    pub fn chunk_file(&self, file_path: &Path) -> Result<(Vec<UltraCdcChunk>, String)> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut chunks = Vec::new();
        let mut offset = 0;
        let len = buffer.len();

        let mut merkle_hasher = md5::Md5::default();

        while offset < len {
            let mut chunk_len = self.min_size.min(len - offset);
            let mut fingerprint: u32 = 0;

            while offset + chunk_len < len && chunk_len < self.max_size {
                let byte = buffer[offset + chunk_len];
                fingerprint = (fingerprint << 1).wrapping_add(byte as u32);
                let mask = if chunk_len < self.avg_size {
                    self.primary_mask
                } else {
                    self.secondary_mask
                };
                if (fingerprint & mask) == 0 {
                    break;
                }
                chunk_len += 1;
            }

            let slice = &buffer[offset..offset + chunk_len];
            let mut hasher = md5::Md5::default();
            use md5::Digest;
            hasher.update(slice);
            let hash_bytes = hasher.finalize();
            let hash_str = hex::encode(hash_bytes);

            merkle_hasher.update(hash_bytes);

            chunks.push(UltraCdcChunk {
                offset: offset as u64,
                length: chunk_len,
                chunk_hash: hash_str,
            });

            offset += chunk_len;
        }

        use md5::Digest;
        let merkle_root = hex::encode(merkle_hasher.finalize());

        Ok((chunks, merkle_root))
    }
}
