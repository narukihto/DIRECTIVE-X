use candle_core::{DType, Device, Result as CandleResult, Tensor};
use candle_nn::{Linear, Module, VarBuilder, VarMap, SGD};
use log::info;

/// شبكة عصبية رمزية منخفضة المستوى قائمة على Candle لمعالجة مصفوفات الأكواد
pub struct CandleNetwork {
    pub device: Device,
    pub fc1: Linear,
    pub fc2: Linear,
    pub embedding_dim: usize,
    pub varmap: VarMap,
    pub optimizer: SGD, // 1. إضافة المُحسن كعنصر أساسي في هيكل الشبكة
}

impl CandleNetwork {
    /// بناء وتجهيز الطبقات العصبية والمصفوفات على المعالج أو البطاقة
    pub fn new(embedding_dim: usize, hidden_dim: usize) -> CandleResult<Self> {
        let device = Device::Cpu; // يمكن تخصيصها لـ CUDA أو Metal عند توفرها
        let varmap = VarMap::new();
        let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        let fc1 = candle_nn::linear(embedding_dim, hidden_dim, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(hidden_dim, embedding_dim, vs.pp("fc2"))?;

        // 2. تهيئة مُحسن الـ SGD وربطه بمتغيرات الـ VarMap ومعدل تعلم (Learning Rate) مناسب
        let learning_rate = 0.01;
        let optimizer = SGD::new(varmap.all_vars(), learning_rate)?;

        info!(
            "[CANDLE NETWORK] Neural symbolic matrix initialized. Dim: {}, Hidden: {}",
            embedding_dim, hidden_dim
        );

        Ok(Self {
            device,
            fc1,
            fc2,
            embedding_dim,
            varmap,
            optimizer, // قفل كائن المحسن داخل الهيكل
        })
    }

    /// التمرير الأمامي للتنسورات عبر الطبقات العصبية (Forward Pass)
    pub fn forward(&self, input_tensor: &Tensor) -> CandleResult<Tensor> {
        let hidden = self.fc1.forward(input_tensor)?;
        let relu_out = hidden.relu()?;
        let output = self.fc2.forward(&relu_out)?;
        Ok(output)
    }

    /// تنفيذ دورة تدريبية وحساب التدرجات وتحديث الأوزان (Active Training Epoch)
    pub fn train_epoch(&mut self) -> CandleResult<f32> {
        // ملحوظة: في خطوة main.rs القادمة سنستبدل الـ dummy_data ببيانات الـ DataLoader الحية
        let dummy_data = vec![0.5f32; self.embedding_dim];
        let input = Tensor::from_slice(&dummy_data, (1, self.embedding_dim), &self.device)?;
        let target = Tensor::from_slice(&dummy_data, (1, self.embedding_dim), &self.device)?;

        let output = self.forward(&input)?;
        let diff = (output - target)?;
        let loss = diff.sqr()?.mean_all()?;

        // 3. السحر الهندسي: تصفير التدرجات السابقة وحساب التدرجات العكسية الجديدة
        let loss_val = loss.to_scalar::<f32>()?;
        
        // حساب التراجع العكسي للتدرجات بناءً على قيمة الـ Loss
        loss.backward()?; 
        
        // خطوة التحديث الفعلي للأوزان داخل الـ safetensors
        self.optimizer.step()?; 

        Ok(loss_val)
    }

    /// تحويل مصفوفة البايتات الخام القادمة من DataLoader إلى تنسور مباشر
    pub fn raw_bytes_to_tensor(&self, raw_data: &[f32]) -> CandleResult<Tensor> {
        let tensor = Tensor::from_slice(raw_data, (1, raw_data.len()), &self.device)?;
        Ok(tensor)
    }
}
