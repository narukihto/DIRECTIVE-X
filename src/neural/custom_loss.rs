use candle_core::{Result as CandleResult, Tensor};
use log::info;
use num_bigint::BigUint;

use crate::core::causal_system::{CausalCollapseSystem, QuantumNode};

/// دالة خسارة مخصصة تربط خوارزمية الانهيار السببي الحتمي بالتنسورات العصبية
pub struct CausalCollapseLoss {
    pub threshold_limit: f64,
}

impl CausalCollapseLoss {
    pub fn new(threshold_limit: f64) -> Self {
        Self { threshold_limit }
    }

    /// حساب الخسارة عن طريق قياس انحراف قيم التنسور عن المسار الترددي الحتمي
    pub fn compute_loss(
        &self,
        predicted_tensor: &Tensor,
        target_tensor: &Tensor,
    ) -> CandleResult<Tensor> {
        // 1. حساب متوسط الفرق المربع التقليدي (MSE Base)
        let diff = predicted_tensor.sub(target_tensor)?;
        let mse_loss = diff.sqr()?.mean_all()?;

        // 2. استخراج التنسورات كقيمة مصفوفة لتمريرها محلياً في المحرك السببي
        let pred_vec: Vec<f32> = predicted_tensor.to_vec1()?;
        
        // تحويل مخرجات التنسور إلى عقد ترددية (Quantum Nodes)
        let nodes: Vec<QuantumNode> = pred_vec
            .iter()
            .enumerate()
            .map(|(idx, &val)| QuantumNode {
                id: idx,
                energy_scale: BigUint::from((val.abs() * 1000.0) as u64 + 1),
                frequency: val as f64,
            })
            .collect();

        // 3. تنفيذ الانهيار السببي للحصول على مسار التصفية الحتمي O(N)
        let system = CausalCollapseSystem::new(nodes);
        let collapse_route = system.execute_collapse();

        // حساب عامل عقوبة الانحراف (Penalty Factor) بناءً على عدد العقد المطروضة من الانهيار
        let total_nodes = pred_vec.len();
        let accepted_nodes = collapse_route.len();
        let rejection_ratio = if total_nodes > 0 {
            (total_nodes - accepted_nodes) as f32 / total_nodes as f32
        } else {
            0.0
        };

        // 4. دمج العقوبة الترددية مع MSE Loss لمنع الهلوسة في كود الذكاء الاصطناعي
        let penalty_tensor = Tensor::new(rejection_ratio, predicted_tensor.device())?;
        let final_loss = mse_loss.add(&penalty_tensor)?;

        info!(
            "[CUSTOM LOSS] Computed deterministic loss penalty. Total Nodes: {}, Accepted Route: {}, Rejection Ratio: {:.4}",
            total_nodes, accepted_nodes, rejection_ratio
        );

        Ok(final_loss)
    }
}
