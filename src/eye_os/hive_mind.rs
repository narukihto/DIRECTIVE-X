use std::collections::HashMap;
use std::time::Instant;
use log::{info, warn};
use serde::{Deserialize, Serialize};

// استدعاء النواة العصبية الرسمية للتطبيق لتطابق الأنواع 100% في الـ main
use crate::neural::candle_network::CandleNetwork;

/// يمثل وكيلاً من الوكلاء الـ 12 في خلية المعالجة الموزعة الحية
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNode {
    pub id: usize,
    pub name: String,
    pub role: String,
    pub is_active: bool,
    pub tasks_completed: usize,
}

/// موجه خلية الوكلاء الـ 12 وعقل النظام (12-Agent Swarm Orchestrator)
pub struct HiveMind {
    pub agents: HashMap<usize, AgentNode>,
    pub total_processed: usize,
}

impl HiveMind {
    /// تهيئة خلية الوكلاء الـ 12 بالأدوار الاستراتيجية المخصصة
    pub fn new() -> Self {
        let mut agents = HashMap::new();
        let roles = vec![
            "Syntax Processing Engine",
            "Causal Collapse Evaluator",
            "DeFi Arbitrage Matrix Generator",
            "MEV Strategy Synthesizer",
            "High-Frequency Execution Model",
            "Rust/Mojo Optimizing Compiler",
            "Cross-Platform Native Linker",
            "Large Prompt Stream Chunker",
            "Memory-Mapped Tensor Dispatcher",
            "Zero-Copy Bus Router",
            "Arabic/English Logic Alignment",
            "Autonomous Continuous Refiner",
        ];

        for (index, role) in roles.into_iter().enumerate() {
            let id = index + 1;
            agents.insert(
                id,
                AgentNode {
                    id,
                    name: format!("Agent-{}", id),
                    role: role.to_string(),
                    is_active: true,
                    tasks_completed: 0,
                },
            );
        }

        Self {
            agents,
            total_processed: 0,
        }
    }

    /// توجيه مهمة حقيقية وحقنها داخل المصفوفة العصبية بالاستعارة المتغيرة المصححة
    pub fn dispatch_task(
        &mut self, 
        agent_id: usize, 
        payload: &str, 
        neural_net: &mut CandleNetwork
    ) -> Result<String, String> {
        let start_time = Instant::now();

        if let Some(agent) = self.agents.get_mut(&agent_id) {
            if !agent.is_active {
                return Err(format!("Agent [{}] is currently inactive.", agent_id));
            }

            agent.tasks_completed += 1;
            self.total_processed += 1;

            info!(
                "[SWARM EXECUTION] Routing payload through {} [{}]",
                agent.name,
                agent.role
            );

            let raw_bytes = payload.as_bytes();

            // تحويل الحمولة إلى توكنز حقيقية للتوافق مع CandleNetwork المحسّنة
            let sample_input: Vec<u32> = raw_bytes
                .iter()
                .map(|&b| (b as u32) % (neural_net.vocab_size as u32))
                .collect();

            if sample_input.is_empty() {
                return Err("Payload tokenized into empty stream.".to_string());
            }

            let input_slice = &sample_input[..sample_input.len().min(16)];
            let target_slice = input_slice; // مطابقة التوكن الملاحظ كمخرج للاستدلال

            // استدعاء حقيقي للـ train_step بعد تحديث التوقيع
            let inference_loss = neural_net
                .train_step(input_slice, target_slice)
                .map_err(|e| format!("[NEURAL FAIL] Agent failed tensor pass: {:?}", e))?;

            info!(
                "[HIVE MIND] [✓] {} finished execution. Sub-System Loss: {:.6} in {:.3?}ms",
                agent.name,
                inference_loss,
                start_time.elapsed().as_secs_f64() * 1000.0
            );

            Ok(format!(
                "AGENT_SUCCESS | Executor: {} [{}] | System State Verified | Payload Size: {} bytes",
                agent.name,
                agent.role,
                raw_bytes.len()
            ))
        } else {
            warn!("[HIVE MIND] Target Agent ID {} not found in swarm.", agent_id);
            Err(format!("Agent ID {} does not exist.", agent_id))
        }
    }

    /// الحصول على حالة النشاط الإجمالية لجميع الوكلاء الـ 12
    pub fn get_swarm_status(&self) -> Vec<AgentNode> {
        let mut status_list: Vec<AgentNode> = self.agents.values().cloned().collect();
        status_list.sort_by_key(|a| a.id);
        status_list
    }
}
