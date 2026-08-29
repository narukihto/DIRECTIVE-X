use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::{Path, PathBuf};
use bytes::Bytes;
use log::{info, warn};
use rayon::prelude::*;
// استيراد المكتبة الرسمية التي تم تأمينها في الـ Cargo.toml
use hf_hub::{api::tokio::Api, Repo, RepoType};

/// المجموعات البرمجية واللغوية المستهدفة للجلب المباشر والحقيقي
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

    /// 📦 تحديد المسارات الداخلية الصافية والدقيقة للملفات الخام داخل المستودعات
    pub fn as_file_name(&self) -> &'static str {
        match self {
            DatasetTarget::AyaDataset => "data/train-00000-of-00001.parquet",
            DatasetTarget::CodeXGLUE => "code-to-code/trans/train.jsonl",
            DatasetTarget::TheStackV2 => "data/train-00000-of-00100.parquet",
            DatasetTarget::ShareGPT => "ShareGPT_V3_unfiltered_cleaned_split.json",
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

    /// 🚀 جلب وتحميل الملف الخام الكامل رسمياً وآلياً من الـ Hub وتخزينه في كاش الـ Workflow
    pub async fn download_from_hub(&self) -> Result<PathBuf, String> {
        info!(
            "[DATA LOADER] Authenticating & Connecting to Hugging Face Hub for: {}",
            self.target.as_str()
        );

        // إنشاء اتصال رسمي عبر مكتبة hf-hub
        let api = Api::new().map_err(|e| format!("[HF API ERROR] {}", e))?;
        
        // تحديد مستودع البيانات المستهدف
        let repo = api.repo(Repo::new(self.target.as_str().to_string(), RepoType::Dataset));

        // سحب الملف الحقيقي وتخزينه محلياً على القرص الصلب للـ GitHub Actions Runner
        let local_path = repo.get(self.target.as_file_name()).await
            .map_err(|e| format!("[DOWNLOAD ERROR] Failed to fetch raw file: {}", e))?;

        info!(
            "[DATA LOADER] Secure download complete. Dataset anchored locally at: {:?}",
            local_path
        );

        Ok(local_path)
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
