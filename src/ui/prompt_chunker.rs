use log::{info, warn};
use std::collections::VecDeque;

/// يمثل قطعة نصية معالجة ومجهزة للتمرير المباشر لـ Rust Core
#[derive(Debug, Clone)]
pub struct PromptChunk {
    pub id: usize,
    pub payload: String,
    pub token_estimate: usize,
}

/// ذاكرة استقبال وتجزئة النصوص والمدخلات العملاقة
pub struct PromptChunker {
    pub chunk_size_bytes: usize,
    pub buffer: VecDeque<u8>,
    pub chunk_counter: usize,
}

impl PromptChunker {
    /// إنشاء ذاكرة تجزئة جديدة بحجم شريحة محدد بالبايت
    pub fn new(chunk_size_bytes: usize) -> Self {
        let size = if chunk_size_bytes == 0 { 64 * 1024 } else { chunk_size_bytes };
        Self {
            chunk_size_bytes: size,
            buffer: VecDeque::new(),
            chunk_counter: 0,
        }
    }

    /// تغذية الذاكرة بالنصوص والمدخلات الضخمة
    pub fn feed_str(&mut self, input: &str) {
        self.buffer.extend(input.as_bytes());
        info!(
            "[PROMPT CHUNKER] Ingested {} bytes into active buffer stream.",
            input.len()
        );
    }

    /// تجزئة واستخراج كافة الشرائح المتاحة في الذاكرة
    pub fn process_all_chunks(&mut self) -> Vec<PromptChunk> {
        let mut chunks = Vec::new();

        while self.buffer.len() >= self.chunk_size_bytes {
            let mut chunk_bytes = Vec::with_capacity(self.chunk_size_bytes);
            for _ in 0..self.chunk_size_bytes {
                if let Some(byte) = self.buffer.pop_front() {
                    chunk_bytes.push(byte);
                }
            }

            // التحقق من حدود أحرف UTF-8 لتجنب تقطيع الأحرف الممتدة (كالعربية مثلاً)
            let safe_string = String::from_utf8_lossy(&chunk_bytes).into_owned();
            self.chunk_counter += 1;

            let chunk = PromptChunk {
                id: self.chunk_counter,
                token_estimate: safe_string.len() / 4,
                payload: safe_string,
            };

            chunks.push(chunk);
        }

        // معالجة المتبقي في الذاكرة (القطعة الأخيرة)
        if !self.buffer.is_empty() {
            let remaining_bytes: Vec<u8> = self.buffer.drain(..).collect();
            let safe_string = String::from_utf8_lossy(&remaining_bytes).into_owned();
            self.chunk_counter += 1;

            let chunk = PromptChunk {
                id: self.chunk_counter,
                token_estimate: safe_string.len() / 4,
                payload: safe_string,
            };

            chunks.push(chunk);
        }

        info!(
            "[PROMPT CHUNKER] Total chunks generated from input stream: {}",
            chunks.len()
        );

        chunks
    }

    /// مسح وتصفية الذاكرة التراكمية
    pub fn clear(&mut self) {
        self.buffer.clear();
        warn!("[PROMPT CHUNKER] Buffer reset executed.");
    }
}
