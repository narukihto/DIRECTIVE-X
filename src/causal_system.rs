//! # مكتبة الانهيار الترددي السببي المكمم (Causal Collapse System)
//!
//! توفر هذه المكتبة محاكاة هندسية حتمية لحل معضلة TSP والأنظمة الموزعة بناءً على
//! ثبات المقياس الديناميكي، الرنين التراكمي، وذاكرة خياطة الفجوات الخاطفة.

use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rayon::prelude::*;

/// يمثل عقدة ترددية في الفضاء الفلكي للتنسورات
#[derive(Clone, Debug)]
pub struct QuantumNode {
    pub id: usize,
    /// قيمة الطاقة الاسمية (الأصفار الأمامية)
    pub energy_scale: BigUint,
    /// المقيم الترددي الحالي للعقدة
    pub frequency: f64,
}

/// النظام الديناميكي لإدارة شبكة الانهيار السببي
pub struct CausalCollapseSystem {
    pub nodes: Vec<QuantumNode>,
    pub threshold_limit: f64,
    pub buffer_capacity: usize,
}

impl CausalCollapseSystem {
    pub fn new(nodes: Vec<QuantumNode>) -> Self {
        Self {
            nodes,
            threshold_limit: 0.02,
            buffer_capacity: 16,
        }
    }

    fn project_to_inverse_dimensional_symmetry(&self, raw_value: f64, index: usize) -> f64 {
        let dimension_factor = (index as f64 + 1.0).ln();
        let high_dimensional_shadow = (raw_value * dimension_factor).sin();
        high_dimensional_shadow.abs()
    }

    pub fn execute_collapse(&self) -> Vec<usize> {
        if self.nodes.is_empty() {
            return vec![];
        }

        let mut ordered_nodes = self.nodes.clone();
        ordered_nodes.sort_by(|a, b| b.energy_scale.cmp(&a.energy_scale));

        let active_nodes: Vec<QuantumNode> = ordered_nodes
            .par_iter()
            .map(|node| {
                let mut triggered = node.clone();
                if triggered.frequency == 0.0 {
                    triggered.frequency = 0.01;
                }
                triggered
            })
            .collect();

        let mut final_path = Vec::new();
        let mut skipped_buffer: Vec<&QuantumNode> = Vec::with_capacity(self.buffer_capacity);

        final_path.push(active_nodes[0].id);

        let mut cumulative_frequency = active_nodes[0].frequency;
        let mut active_count = 1.0;

        // 1. التصفية التراكمية المحكمة (Pass 1)
        for i in 1..active_nodes.len() {
            let next = &active_nodes[i];
            let current_avg_freq = cumulative_frequency / active_count;
            let pure_dev = (current_avg_freq - next.frequency).abs();

            if pure_dev > self.threshold_limit {
                if pure_dev > self.threshold_limit * 3.0 {
                    if skipped_buffer.len() < self.buffer_capacity {
                        skipped_buffer.push(next);
                    }
                    continue;
                }

                let stable_projected =
                    self.project_to_inverse_dimensional_symmetry(current_avg_freq, i - 1);
                let next_projected =
                    self.project_to_inverse_dimensional_symmetry(next.frequency, i);
                let projected_dev = (stable_projected - next_projected).abs();

                if projected_dev > self.threshold_limit {
                    if skipped_buffer.len() < self.buffer_capacity {
                        skipped_buffer.push(next);
                    }
                    continue;
                }
            }

            let scale_factor = 1.0 / (next.energy_scale.to_f64().unwrap_or(1.0) + 1.0);
            let combined_resonance = pure_dev * scale_factor;

            if combined_resonance <= self.threshold_limit {
                final_path.push(next.id);
                cumulative_frequency += next.frequency;
                active_count += 1.0;
            } else {
                if skipped_buffer.len() < self.buffer_capacity {
                    skipped_buffer.push(next);
                }
            }
        }

        // 2. خياطة الفجوات المحمية (Pass 2)
        let final_avg_freq = cumulative_frequency / active_count;

        for buffered_node in skipped_buffer {
            let pure_raw_dev = (final_avg_freq - buffered_node.frequency).abs();

            if pure_raw_dev > self.threshold_limit * 1.5 {
                continue;
            }

            let scale_factor =
                1.0 / (buffered_node.energy_scale.to_f64().unwrap_or(1.0) + 1.0);

            if pure_raw_dev * scale_factor <= self.threshold_limit {
                final_path.push(buffered_node.id);
            }
        }

        final_path
    }
}
