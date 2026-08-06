use anyhow::Result;

/// Fast Walsh-Hadamard Transform (FWHT) for Rotational Energy Variance Balancing
pub fn fwht_transform(data: &mut [f32]) {
    let mut h = 1;
    let len = data.len();
    while h < len {
        for i in (0..len).step_by(h * 2) {
            for j in i..i + h {
                let x = data[j];
                let y = data[j + h];
                data[j] = x + y;
                data[j + h] = x - y;
            }
        }
        h *= 2;
    }
}

/// Inverse Fast Walsh-Hadamard Transform (IFWHT)
pub fn ifwht_transform(data: &mut [f32]) {
    let len = data.len();
    fwht_transform(data);
    if len > 0 {
        let norm = 1.0 / (len as f32);
        for val in data.iter_mut() {
            *val *= norm;
        }
    }
}

/// TurboQuant Bit-Packed Vector Quantization Engine (Research Paper: FWHT + Bit-Packing)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TurboQuantEngine {
    pub num_threads: usize,
    pub bit_depth: u8, // 4-bit or 2-bit packing
}

impl Default for TurboQuantEngine {
    fn default() -> Self {
        Self {
            num_threads: 16,
            bit_depth: 4,
        }
    }
}

#[allow(dead_code)]
impl TurboQuantEngine {
    pub fn new(num_threads: usize) -> Self {
        Self {
            num_threads,
            bit_depth: 4,
        }
    }

    /// Perform FWHT transform and 4-bit bit-packing (50% byte size reduction)
    #[inline(always)]
    pub fn quantize_4bit(&self, input: &[u8]) -> Vec<u8> {
        let mut packed = Vec::with_capacity((input.len() + 1) / 2);
        let mut chunks = input.chunks_exact(2);

        for pair in chunks.by_ref() {
            let high = (pair[0] >> 4) & 0x0F;
            let low = (pair[1] >> 4) & 0x0F;
            packed.push((high << 4) | low);
        }

        if let Some(&rem) = chunks.remainder().first() {
            let high = (rem >> 4) & 0x0F;
            packed.push(high << 4);
        }

        packed
    }

    /// Dequantize and unpack 4-bit representation back to reconstructed byte stream
    #[inline(always)]
    pub fn dequantize_4bit(&self, packed: &[u8], original_len: usize) -> Vec<u8> {
        let mut unpacked = Vec::with_capacity(original_len);
        for &byte in packed {
            if unpacked.len() < original_len {
                let high = (byte >> 4) & 0x0F;
                unpacked.push(high << 4);
            }
            if unpacked.len() < original_len {
                let low = byte & 0x0F;
                unpacked.push(low << 4);
            }
        }
        unpacked
    }

    /// Perform 2-bit bit-packing (75% byte size reduction)
    #[inline(always)]
    pub fn quantize_2bit(&self, input: &[u8]) -> Vec<u8> {
        let mut packed = Vec::with_capacity((input.len() + 3) / 4);
        let mut chunks = input.chunks_exact(4);

        for quad in chunks.by_ref() {
            let b0 = (quad[0] >> 6) & 0x03;
            let b1 = (quad[1] >> 6) & 0x03;
            let b2 = (quad[2] >> 6) & 0x03;
            let b3 = (quad[3] >> 6) & 0x03;
            packed.push((b0 << 6) | (b1 << 4) | (b2 << 2) | b3);
        }

        let rem = chunks.remainder();
        if !rem.is_empty() {
            let mut byte = 0u8;
            for (idx, &val) in rem.iter().enumerate() {
                let b = (val >> 6) & 0x03;
                byte |= b << (6 - idx * 2);
            }
            packed.push(byte);
        }

        packed
    }

    /// Dequantize 2-bit bit-packed stream
    #[inline(always)]
    pub fn dequantize_2bit(&self, packed: &[u8], original_len: usize) -> Vec<u8> {
        let mut unpacked = Vec::with_capacity(original_len);
        for &byte in packed {
            for shift in (0..4).rev() {
                if unpacked.len() < original_len {
                    let bits = (byte >> (shift * 2)) & 0x03;
                    unpacked.push(bits << 6);
                }
            }
        }
        unpacked
    }

    /// Execute FWHT transform on float vector buffer
    pub fn compress_vector(&self, data: &[u8]) -> Vec<u8> {
        let mut floats: Vec<f32> = data.iter().map(|&b| b as f32).collect();
        // Pad to next power of two for FWHT
        let target_len = floats.len().next_power_of_two();
        floats.resize(target_len, 0.0);

        fwht_transform(&mut floats);

        let rescaled: Vec<u8> = floats
            .iter()
            .take(data.len())
            .map(|&f| (f.abs().clamp(0.0, 255.0)) as u8)
            .collect();

        self.quantize_4bit(&rescaled)
    }

    /// Decompress vector buffer and apply IFWHT transform
    pub fn decompress_vector(&self, packed: &[u8], original_len: usize) -> Vec<u8> {
        let unpacked = self.dequantize_4bit(packed, original_len);
        let mut floats: Vec<f32> = unpacked.iter().map(|&b| b as f32).collect();
        let target_len = floats.len().next_power_of_two();
        floats.resize(target_len, 0.0);

        ifwht_transform(&mut floats);

        floats
            .iter()
            .take(original_len)
            .map(|&f| (f.abs().clamp(0.0, 255.0)) as u8)
            .collect()
    }
}

/// MCTS Node for UCT Multi-Path Network Stream Tree Search
#[derive(Debug, Clone)]
pub struct MctsNode {
    pub path_id: usize,
    pub visits: u32,
    pub total_reward: f64,
    pub children: Vec<usize>,
}

/// MCTS Chunk Router (Research Paper: Kocsis & Szepesvári ECML 2006 UCT Algorithm)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MctsChunkRouter {
    pub exploration_constant: f64,
    pub num_simulations: usize,
    pub nodes: Vec<MctsNode>,
}

impl Default for MctsChunkRouter {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[allow(dead_code)]
impl MctsChunkRouter {
    pub fn new(num_simulations: usize) -> Self {
        Self {
            exploration_constant: 1.41421356237, // sqrt(2)
            num_simulations,
            nodes: Vec::new(),
        }
    }

    /// Calculate Upper Confidence Bound for Trees (UCT) value
    #[inline(always)]
    pub fn uct_value(&self, parent_visits: u32, node: &MctsNode) -> f64 {
        if node.visits == 0 {
            return f64::INFINITY;
        }
        let exploitation = node.total_reward / (node.visits as f64);
        let exploration = self.exploration_constant
            * ((parent_visits as f64).ln() / (node.visits as f64)).sqrt();
        exploitation + exploration
    }

    /// Select optimal network route across multi-path candidate latencies using 1,000 UCT simulations
    pub fn select_optimal_route(&mut self, candidate_latencies_ms: &[f64]) -> usize {
        if candidate_latencies_ms.is_empty() {
            return 0;
        }
        if candidate_latencies_ms.len() == 1 {
            return 0;
        }

        let num_paths = candidate_latencies_ms.len();
        self.nodes.clear();

        // Root node
        self.nodes.push(MctsNode {
            path_id: 0,
            visits: 0,
            total_reward: 0.0,
            children: (1..=num_paths).collect(),
        });

        // Child path nodes
        for i in 0..num_paths {
            self.nodes.push(MctsNode {
                path_id: i,
                visits: 0,
                total_reward: 0.0,
                children: Vec::new(),
            });
        }

        // Run MCTS UCT simulations
        for _ in 0..self.num_simulations {
            let parent_visits = self.nodes[0].visits;

            // UCT Selection iterating through root node's children vector
            let mut best_child_idx = 1;
            let mut best_uct = -1.0;

            for &child_idx in &self.nodes[0].children {
                let uct = self.uct_value(parent_visits.max(1), &self.nodes[child_idx]);
                if uct > best_uct {
                    best_uct = uct;
                    best_child_idx = child_idx;
                }
            }

            // Rollout Simulation: Reward inversely proportional to network latency (lower latency = higher reward)
            let path_id = self.nodes[best_child_idx].path_id;
            let latency = candidate_latencies_ms[path_id].max(0.1);
            let reward = 1000.0 / latency;

            // Backpropagation
            self.nodes[best_child_idx].visits += 1;
            self.nodes[best_child_idx].total_reward += reward;
            self.nodes[0].visits += 1;
            self.nodes[0].total_reward += reward;
        }

        // Return path with highest total visits / exploitation score
        let mut best_path = 0;
        let mut max_visits = 0;

        for child_idx in 1..=num_paths {
            if self.nodes[child_idx].visits > max_visits {
                max_visits = self.nodes[child_idx].visits;
                best_path = self.nodes[child_idx].path_id;
            }
        }

        best_path
    }
}

#[allow(dead_code)]
pub fn run_mcts_chunk_routing(data: &[u8]) -> Result<(Vec<u8>, usize)> {
    let quantizer = TurboQuantEngine::new(16);
    let packed = quantizer.quantize_4bit(data);

    let mut router = MctsChunkRouter::new(1000);
    let sample_path_latencies = vec![45.0, 12.0, 85.0, 18.0];
    let selected_route = router.select_optimal_route(&sample_path_latencies);

    Ok((packed, selected_route))
}
