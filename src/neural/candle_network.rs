use candle_core::{DType, Device, Result as CandleResult, Tensor};
use candle_nn::{Embedding, Linear, Module, VarBuilder, VarMap, SGD, Optimizer};
use log::info;

/// شبكة عصبية رمزية كاملة قائمة على Candle ومجهزة للترجمة الحقيقية إلى GGUF
pub struct CandleNetwork {
    pub device: Device,
    pub token_embd: Embedding,
    pub fc1: Linear,
    pub fc2: Linear,
    pub lm_head: Linear,
    pub embedding_dim: usize,
    pub vocab_size: usize,
    pub varmap: VarMap,
    pub optimizer: SGD,
}

impl CandleNetwork {
    /// بناء وتجهيز الطبقات العصبية والمصفوفات شاملاً التضمين والمفردات
    pub fn new(vocab_size: usize, embedding_dim: usize, hidden_dim: usize) -> CandleResult<Self> {
        let device = Device::Cpu; 
        let varmap = VarMap::new();
        let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        // 1. طبقة التضمين لتأطير التوكنز (token_embd)
        let token_embd = candle_nn::embedding(vocab_size, embedding_dim, vs.pp("token_embd"))?;
        
        // 2. الطبقات العصبية البينية (fc1, fc2)
        let fc1 = candle_nn::linear(embedding_dim, hidden_dim, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(hidden_dim, embedding_dim, vs.pp("fc2"))?;
        
        // 3. طبقة التنبؤ النهائي بالتوكنز (output / lm_head)
        let lm_head = candle_nn::linear(embedding_dim, vocab_size, vs.pp("output"))?;

        let learning_rate = 0.01;
        let optimizer = SGD::new(varmap.all_vars(), learning_rate)?;

        info!(
            "[CANDLE NETWORK] Initialized complete model matrix. Vocab: {}, Dim: {}, Hidden: {}",
            vocab_size, embedding_dim, hidden_dim
        );

        Ok(Self {
            device,
            token_embd,
            fc1,
            fc2,
            lm_head,
            embedding_dim,
            vocab_size,
            varmap,
            optimizer, 
        })
    }

    /// التمرير الأمامي: يستقبل التوكنز ويحسب الاحتمالات على المفردات (Logits)
    pub fn forward(&self, input_ids: &Tensor) -> CandleResult<Tensor> {
        let embed = self.token_embd.forward(input_ids)?;
        let hidden = self.fc1.forward(&embed)?;
        let relu_out = hidden.relu()?;
        let dense = self.fc2.forward(&relu_out)?;
        let logits = self.lm_head.forward(&dense)?;
        Ok(logits)
    }

    /// تنفيذ دورة تدريبية وحساب التدرجات مع التوكنز الحقيقية
    pub fn train_step(&mut self, input_tokens: &[u32], target_tokens: &[u32]) -> CandleResult<f32> {
        let input_tensor = Tensor::new(input_tokens, &self.device)?.unsqueeze(0)?;
        let target_tensor = Tensor::new(target_tokens, &self.device)?;

        let logits = self.forward(&input_tensor)?;
        let logits_flat = logits.squeeze(0)?;
        
        // حساب Cross Entropy Loss القياسي لشبكات التوليد
        let loss = candle_nn::loss::cross_entropy(&logits_flat, &target_tensor)?;
        let loss_val = loss.to_scalar::<f32>()?;

        let grads = loss.backward()?; 
        self.optimizer.step(&grads)?; 

        Ok(loss_val)
    }

    /// حفظ كافة الأوزان الحقيقية المحدثة بما فيها التضمين والمفردات
    pub fn save_weights(&self, path: &str) -> CandleResult<()> {
        self.varmap.save(path)?;
        info!("[CANDLE NETWORK] Successfully saved all model layers to {}", path);
        Ok(())
    }
}
