use candle_core::{DType, Device, Result as CandleResult, Tensor};
use candle_nn::{Linear, Module, VarBuilder, VarMap};
use log::info;

/// شبكة عصبية رمزية منخفضة المستوى قائمة على Candle لمعالجة مصفوفات الأكواد
pub struct CandleNetwork {
    pub device: Device,
    pub fc1: Linear,
    pub fc2: Linear,
    pub embedding_dim: usize,
}

impl CandleNetwork {
    /// بناء وتجهيز الطبقات العصبية والمصفوفات على المعالج أو البطاقة
    pub fn new(embedding_dim: usize, hidden_dim: usize) -> CandleResult<Self> {
        let device = Device::Cpu; // يمكن تخصيصها لـ CUDA أو Metal عند توفرها
        let mut varmap = VarMap::new();
        let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        let fc1 = candle_nn::linear(embedding_dim, hidden_dim, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(hidden_dim, embedding_dim, vs.pp("fc2"))?;

        info!(
            "[CANDLE NETWORK] Neural symbolic matrix initialized. Dim: {}, Hidden: {}",
            embedding_dim, hidden_dim
        );

        Ok(Self {
            device,
            fc1,
            fc2,
            embedding_dim,
        })
    }

    /// التمرير الأمامي للتنسورات عبر الطبقات العصبية (Forward Pass)
    pub fn forward(&self, input_tensor: &Tensor) -> CandleResult<Tensor> {
        let hidden = self.fc1.forward(input_tensor)?;
        let relu_out = hidden.relu()?;
        let output = self.fc2.forward(&relu_out)?;
        Ok(output)
    }

    /// تحويل مصفوفة البايتات الخام القادمة من DataLoader إلى تنسور مباشر
    pub fn raw_bytes_to_tensor(&self, raw_data: &[f32]) -> CandleResult<Tensor> {
        let tensor = Tensor::from_slice(raw_data, (1, raw_data.len()), &self.device)?;
        Ok(tensor)
    }
}
