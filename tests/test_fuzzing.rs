use num_bigint::BigUint;
use std::time::Instant;

// استدعاء الوحدات المستهدفة لاختبار المتانة
#[path = "../src/core/mod.rs"]
mod core;
#[path = "../src/ui/mod.rs"]
mod ui;

use crate::core::causal_system::{CausalCollapseSystem, QuantumNode};
use crate::ui::prompt_chunker::PromptChunker;

/// توليد مصفوفات بايتات عشوائية لمحاكاة الفلترة غير المتوقعة (Pseudo-random byte generator)
fn generate_fuzz_bytes(seed: usize, len: usize) -> Vec<u8> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            (state >> 16) as u8
        })
        .collect()
}

#[test]
fn fuzz_prompt_chunker_with_malformed_utf8() {
    println!("[FUZZ TEST] Starting Prompt Chunker resilience test with malformed UTF-8 stream...");
    let mut chunker = PromptChunker::new(1024);

    // تغذية الذاكرة بـ 500 كيلوبايت من البايتات العشوائية المكسورة
    let corrupted_bytes = generate_fuzz_bytes(0xDEADBEEF, 500 * 1024);
    let lossy_string = String::from_utf8_lossy(&corrupted_bytes);

    chunker.feed_str(&lossy_string);
    let chunks = chunker.process_all_chunks();

    assert!(!chunks.is_empty(), "Chunker failed to extract chunks from fuzz stream!");
    println!(
        "[FUZZ TEST] Prompt Chunker successfully handled corrupted stream. Total generated chunks: {}",
        chunks.len()
    );
}

#[test]
fn fuzz_causal_collapse_system_edge_cases() {
    println!("[FUZZ TEST] Executing Causal Collapse edge case & overflow test...");

    // 1. اختبار حافة المصفوفة الفارغة (Zero Elements)
    let empty_engine = CausalCollapseSystem::new(vec![]);
    let empty_result = empty_engine.execute_collapse();
    assert!(empty_result.is_empty(), "Collapse engine must return empty route for empty nodes.");

    // 2. اختبار القيم الترددية الشديدة والحواف القريبة من الصفر (Extreme Frequencies & Energies)
    let start_time = Instant::now();
    let fuzz_nodes: Vec<QuantumNode> = (0..5000)
        .map(|idx| {
            let bytes = generate_fuzz_bytes(idx, 8);
            let energy_val = u64::from_le_bytes(bytes.try_into().unwrap());
            QuantumNode {
                id: idx,
                energy_scale: BigUint::from(energy_val),
                frequency: if idx % 2 == 0 { 0.0 } else { (idx as f64) * 1e-10 },
            }
        })
        .collect();

    let fuzz_engine = CausalCollapseSystem::new(fuzz_nodes);
    let route = fuzz_engine.execute_collapse();

    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    println!(
        "[FUZZ TEST] Causal Collapse completed fuzz route of 5000 nodes in {:.3}ms. Selected route nodes: {}",
        elapsed,
        route.len()
    );
    assert!(!route.is_empty(), "Engine failed to build deterministic route under fuzz load!");
}
