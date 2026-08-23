use std::collections::HashMap;
use std::time::Instant;
use log::{info, warn};
use serde::{Deserialize, Serialize};

/// يمثل وكيلاً من الوكلاء الـ 12 في خلية المعالجة الموزعة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNode {
    pub id: usize,
    pub name: String,
    pub role: String,
    pub is_active: bool,
    pub tasks_completed: usize,
}

/// موجه خلية الوكلاء الـ 12 (12-Agent Swarm Orchestrator)
pub struct HiveMind {
    pub agents: HashMap<usize, AgentNode>,
    pub total_processed: usize,
}

impl HiveMind {
    /// تهيئة خلية الوكلاء الـ 12 الأدوار المخصصة
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
                    name: format!("Agent-{:02}", id),
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

    /// توجيه مهمة جديدة وتوزيعها عبر الخلية
    pub fn dispatch_task(&mut self, agent_id: usize, payload: &str) -> Result<String, String> {
        let start_time = Instant::now();

        if let Some(agent) = self.agents.get_mut(&agent_id) {
            if !agent.is_active {
                return Err(format!("Agent [{}] is currently inactive.", agent_id));
            }

            agent.tasks_completed += 1;
            self.total_processed += 1;

            info!(
                "[HIVE MIND] Dispatched payload to {} [{}] in {:.3?}ms",
                agent.name,
                agent.role,
                start_time.elapsed().as_secs_f64() * 1000.0
            );

            Ok(format!(
                "SUCCESS: Executed by {} | Result generated for context chunk length {}",
                agent.name,
                payload.len()
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
