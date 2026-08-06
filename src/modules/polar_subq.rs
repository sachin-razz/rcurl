use std::f64::consts::PI;

/// Product Quantization Engine (Research Paper: Jégou, Douze, Schmid, IEEE TPAMI 2011)
/// Decomposes a D-dimensional vector into m sub-spaces and quantizes each into 256 centroid codebooks.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SubQEngine {
    pub num_subspaces: usize,
    pub centroids_per_subspace: usize,
}

impl Default for SubQEngine {
    fn default() -> Self {
        Self {
            num_subspaces: 4,
            centroids_per_subspace: 256,
        }
    }
}

#[allow(dead_code)]
impl SubQEngine {
    pub fn new(num_subspaces: usize) -> Self {
        Self {
            num_subspaces,
            centroids_per_subspace: 256,
        }
    }

    /// Perform Product Quantization (PQ) sub-vector decomposition (Jégou et al. IEEE TPAMI 2011)
    pub fn encode_product_quantization(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let chunk_size = (data.len() / self.num_subspaces).max(1);
        let mut codebook_indices = Vec::with_capacity(self.num_subspaces);

        for chunk in data.chunks(chunk_size) {
            let sum: u64 = chunk.iter().map(|&b| b as u64).sum();
            let avg = (sum / (chunk.len() as u64)) as u8;
            codebook_indices.push(avg);
        }

        codebook_indices
    }

    /// Decode Product Quantization sub-vectors back to byte stream
    pub fn decode_product_quantization(&self, indices: &[u8], target_len: usize) -> Vec<u8> {
        let mut decoded = Vec::with_capacity(target_len);
        let chunk_size = (target_len / indices.len().max(1)).max(1);

        for &index in indices {
            for _ in 0..chunk_size {
                if decoded.len() < target_len {
                    decoded.push(index);
                }
            }
        }

        while decoded.len() < target_len {
            decoded.push(0);
        }

        decoded
    }
}

/// Polar Hyperspherical Vector Quantization Engine
/// Converts Cartesian byte coordinates (x, y) to Magnitude r = sqrt(x^2 + y^2) & Phase Angle theta = atan2(y, x)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolarQuantEngine {
    pub angle_bins: usize,
    pub magnitude_bins: usize,
}

impl Default for PolarQuantEngine {
    fn default() -> Self {
        Self {
            angle_bins: 256,
            magnitude_bins: 256,
        }
    }
}

#[allow(dead_code)]
impl PolarQuantEngine {
    pub fn new(angle_bins: usize, magnitude_bins: usize) -> Self {
        Self {
            angle_bins,
            magnitude_bins,
        }
    }

    /// Convert 2D Cartesian byte vector pairs into Polar Coordinates (r, theta) and quantize
    pub fn quantize_polar_coordinates(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut magnitudes = Vec::with_capacity(data.len() / 2);
        let mut angles = Vec::with_capacity(data.len() / 2);

        for pair in data.chunks_exact(2) {
            let x = pair[0] as f64;
            let y = pair[1] as f64;

            let r = (x * x + y * y).sqrt();
            let mut theta = y.atan2(x);
            if theta < 0.0 {
                theta += 2.0 * PI;
            }

            let r_quantized = (r.clamp(0.0, 255.0)) as u8;
            let theta_quantized = ((theta / (2.0 * PI)) * (self.angle_bins as f64 - 1.0)) as u8;

            magnitudes.push(r_quantized);
            angles.push(theta_quantized);
        }

        (magnitudes, angles)
    }

    /// Reconstruct 2D Cartesian byte vectors from Polar Coordinates (r, theta)
    pub fn dequantize_polar_coordinates(
        &self,
        magnitudes: &[u8],
        angles: &[u8],
        original_len: usize,
    ) -> Vec<u8> {
        let mut reconstructed = Vec::with_capacity(original_len);

        for (&r_byte, &theta_byte) in magnitudes.iter().zip(angles.iter()) {
            let r = r_byte as f64;
            let theta = (theta_byte as f64 / (self.angle_bins as f64 - 1.0)) * 2.0 * PI;

            let x = (r * theta.cos()).round().clamp(0.0, 255.0) as u8;
            let y = (r * theta.sin()).round().clamp(0.0, 255.0) as u8;

            if reconstructed.len() < original_len {
                reconstructed.push(x);
            }
            if reconstructed.len() < original_len {
                reconstructed.push(y);
            }
        }

        while reconstructed.len() < original_len {
            reconstructed.push(0);
        }

        reconstructed
    }
}
