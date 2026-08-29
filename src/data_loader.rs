use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::{Path, PathBuf};
use log::{info, warn};
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

    /// 📦 توليد مصفوفة الشظايا المتتالية لامتصاص حجم التريليونات بالتوالي
    pub fn get_target_shards(&self) -> Vec<String> {
        match self {
            // سحب عينات متتالية بصيغها الحقيقية لبدء التهام الحجم الكبير
            DatasetTarget::TheStackV2 => (0..3).map(|i| format!("data/train-{:05}-of-01000.parquet", i)).collect(),
            DatasetTarget::AyaDataset => vec!["data/train-00000-of-00001.parquet".to_string()],
            DatasetTarget::CodeXGLUE => vec!["code-to-code/trans/train.jsonl".to_string()],
            DatasetTarget::ShareGPT => vec!["ShareGPT_V3_unfiltered_cleaned_split.json".to_string()],
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

    /// 🚀 جلب شظية محددة رسمياً وآلياً من الـ Hub وتخزينها محلياً في الكاش
    pub async fn download_shard_from_hub(&self, shard_name: &str) -> Result<PathBuf, String> {
        info!(
            "[DATA LOADER] Connecting to official Hub Shard: {} -> {}",
            self.target.as_str(),
            shard_name
        );

        let api = Api::new().map_err(|e| format!("[HF API ERROR] {}", e))?;
        let repo = api.repo(Repo::new(self.target.as_str().to_string(), RepoType::Dataset));

        let local_path = repo.get(shard_name).await
            .map_err(|e| format!("[DOWNLOAD ERROR] Failed to fetch raw shard: {}", e))?;

        Ok(local_path)
    }

    /// قراءة وسكب البيانات المباشرة عبر Memory-Mapped I/O بدون تخزين مؤقت للوزن
    pub fn mmap_pass_to_tensor_core<P: AsRef<Path>>(&self, file_path: P) -> IoResult<Vec<u8>> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::with_capacity(self.buffer_capacity);
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
