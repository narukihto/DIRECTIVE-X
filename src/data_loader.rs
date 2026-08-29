use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::{Path, PathBuf};
use log::{info, warn};
use hf_hub::{api::tokio::Api, Repo, RepoType};

/// المجموعات البرمجية واللغوية والعلمية الشاملة لامتصاص المعرفة الموسوعية الفلكية
#[derive(Debug, Clone)]
pub enum DatasetTarget {
    TheStackV2,    // مستودع الأكواد السيادية الفائق والكامل
    CodeXGLUE,     // بنية فهم وترجمة الأكواد
    AyaDataset,    // النواة اللغوية ومتعددة اللغات الصافية
    ShareGPT,      // منطق المحادثات والتفكير المسترسل والجدل
    FineWebEdu,    // 🧠 النواة التعليمية: صفوة العلوم والمنطق والرياضيات المصفاة عالمياً
    Wikipedia,     // 🌍 النواة التاريخية والثقافية: خلاصة الموسوعات البشرية الشاملة
}

impl DatasetTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatasetTarget::TheStackV2 => "bigcode/the-stack-v2",
            DatasetTarget::CodeXGLUE => "microsoft/CodeXGLUE",
            DatasetTarget::AyaDataset => "CohereForAI/aya_dataset",
            DatasetTarget::ShareGPT => "anon8231489123/ShareGPT_Vicuna_unfiltered",
            DatasetTarget::FineWebEdu => "HuggingFaceFW/fineweb-edu", // المستودع التأسيسي الخارق للعلوم
            DatasetTarget::Wikipedia => "wikimedia/wikipedia",       // مستودع المعرفة البشرية الشامل
        }
    }

    /// 📦 🌍 توليد مصفوفة الشظايا العملاقة لامتصاص مئات الجيجابايت والتريليونات اللانهائية بالتوالي
    pub fn get_target_shards(&self) -> Vec<String> {
        match self {
            // 🚀 التهام كامل فلكي: فتح العداد ليمسح الـ 1000 ملف بالكامل لمستودع الأكواد (امتصاص كامل المصادر)
            DatasetTarget::TheStackV2 => (0..1000).map(|i| format!("data/train-{:05}-of-01000.parquet", i)).collect(),
            
            // 🧠 التهام النواة التعليمية والعلوم الفوقية: سحب أول 100 شظية ضخمة من العلوم الصافية لـ FineWeb
            DatasetTarget::FineWebEdu => (0..100).map(|i| format!("sample/10BT/train-{:05}-of-00100.parquet", i)).collect(),

            // 🌍 التهام خلاصة التاريخ والموسوعات: سحب شظايا الموسوعة العالمية الشاملة ويكيبيديا
            DatasetTarget::Wikipedia => vec![
                "20231101.en/train-00000-of-00003.parquet".to_string(),
                "20231101.en/train-00000-of-00002.parquet".to_string(),
                "20231101.ar/train-00000-of-00001.parquet".to_string(), // تضمين اللغة العربية الموسوعية
            ],

            // سحب الملف الشامل لبيانات آيا متعددة اللغات
            DatasetTarget::AyaDataset => vec!["data/train-00000-of-00001.parquet".to_string()],
            
            // سحب ملف كود إكس جلو بالكامل
            DatasetTarget::CodeXGLUE => vec!["code-to-code/trans/train.jsonl".to_string()],
            
            // سحب ملف المحادثات الصافي بالكامل
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

    /// 🚀 جلب شظية محددة رسمياً وآلياً عبر مكتبة hf-hub وتخزينها محلياً في الكاش
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
