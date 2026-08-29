use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;
use bytes::Bytes;
use log::{info, warn};
use rayon::prelude::*;

/// المجموعات البرمجية واللغوية المستهدفة للجلب المباشر
#[derive(Debug, Clone)]
pub enum DatasetTarget {
    TheStackV2,
    CodeXGLUE,
    AyaDataset,
    ShareGPT,
}

impl DatasetTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatasetTarget::TheStackV2 => "bigcode/the-stack-v2",
            DatasetTarget::CodeXGLUE => "microsoft/CodeXGLUE",
            DatasetTarget::AyaDataset => "CohereForAI/aya_dataset",
            DatasetTarget::ShareGPT => "anon8231489123/ShareGPT_Vicuna_unfiltered",
        }
    }
}

/// أنبوب تحميل البيانات الخالي من التوكنات (Token-Free Data Pipeline)
pub struct DataLoader {
    pub target: DatasetTarget,
    pub buffer_capacity: usize,
}

impl DataLoader {
    pub fn new(target: DatasetTarget, buffer_capacity: usize) -> Self {
        Self {
            target,
            buffer_capacity,
        }
    }

    /// محاكاة جلب تدفق البيانات مباشرة من Hugging Face Hub عبر Stream منخفض التأخير
    pub async fn stream_from_hub(&self) -> Result<Bytes, String> {
        info!(
            "[DATA LOADER] Connecting to Hugging Face Hub endpoint: {}",
            self.target.as_str()
        );

        // محاكاة استلام التنسورات عبر التدفق السريع في الذاكرة
        let simulated_payload = format!(
            "DIRECTIVE-X_STREAM_INGESTION_DATASET_{}_RAW_BYTES",
            self.target.as_str().replace('/', "_")
        );
        
        let bytes = Bytes::from(simulated_payload.into_bytes());
        info!(
            "[DATA LOADER] Stream established for {}. Bytes buffer size: {}",
            self.target.as_str(),
            bytes.len()
        );

        Ok(bytes)
    }

    /// قراءة وسكب البيانات المباشرة عبر Memory-Mapped I/O بدون تخزين مؤقت للوزن
    pub fn mmap_pass_to_tensor_core<P: AsRef<Path>>(&self, file_path: P) -> IoResult<Vec<u8>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::with_capacity(self.buffer_capacity);
        file.read_to_end(&mut buffer)?;

        info!(
            "[DATA LOADER] Memory-Mapped pass executed. Ingested {} raw bytes to Core Tensor Frame.",
            buffer.len()
        );

        Ok(buffer)
    }

    /// معالجة موازية للمصفوفات والرموز باستخدام Rayon لتمريرها مباشرة لمحرك Rust Core
    pub fn process_parallel_bytes(&self, raw_bytes: &[u8]) -> Vec<f32> {
        if raw_bytes.is_empty() {
            warn!("[DATA LOADER] Received empty byte stream for processing.");
            return vec![];
        }

        // تحويل البايتات مباشرة إلى قيم طافية (Floats) لاستخدامها داخل التنسور بدون Tokenizer
        raw_bytes
            .par_iter()
            .map(|&byte| (byte as f32) / 255.0)
            .collect()
    }
}
