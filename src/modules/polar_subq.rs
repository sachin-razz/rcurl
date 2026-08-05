use anyhow::Result;

/// SubQ (Sub-Vector Quantization) Vector Chunk representation
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SubQChunk {
    pub sub_vector_dim: usize,
    pub quantized_codes: Vec<u8>,
    pub codebook_indices: Vec<u16>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SubQEngine {
    pub sub_vector_size: usize,
}

impl Default for SubQEngine {
    fn default() -> Self {
        Self { sub_vector_size: 4 }
    }
}

#[allow(dead_code)]
impl SubQEngine {
    pub fn new(sub_vector_size: usize) -> Self {
        Self { sub_vector_size }
    }

    /// Perform Sub-Vector Quantization (SubQ) compression on byte data
    pub fn quantize(&self, data: &[u8]) -> SubQChunk {
        let sub_dim = self.sub_vector_size.max(1);
        let mut codes = Vec::new();
        let mut indices = Vec::new();

        for chunk in data.chunks(sub_dim) {
            let sum: u32 = chunk.iter().map(|&b| b as u32).sum();
            let avg = (sum / chunk.len() as u32) as u8;
            codes.push(avg);
            indices.push((avg as u16) ^ (chunk.len() as u16));
        }

        SubQChunk {
            sub_vector_dim: sub_dim,
            quantized_codes: codes,
            codebook_indices: indices,
        }
    }
}

/// PolarQuant (Polar Coordinate Angle-Magnitude Quantization) Vector Chunk representation
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolarQuantChunk {
    pub magnitude: f32,
    pub quantized_angles: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolarQuantEngine {
    pub angle_bins: u16,
}

impl Default for PolarQuantEngine {
    fn default() -> Self {
        Self { angle_bins: 256 }
    }
}

#[allow(dead_code)]
impl PolarQuantEngine {
    pub fn new(angle_bins: u16) -> Self {
        Self { angle_bins }
    }

    /// Perform Polar Coordinate Quantization (PolarQuant) on byte data
    pub fn quantize(&self, data: &[u8]) -> Result<PolarQuantChunk> {
        let sum_sq: f64 = data.iter().map(|&b| (b as f64) * (b as f64)).sum();
        let magnitude = sum_sq.sqrt() as f32;

        let angles = data
            .iter()
            .map(|&b| {
                let norm = if magnitude == 0.0 { 0.0 } else { (b as f32) / magnitude };
                (norm.clamp(0.0, 1.0) * 255.0) as u8
            })
            .collect();

        Ok(PolarQuantChunk {
            magnitude,
            quantized_angles: angles,
        })
    }
}
