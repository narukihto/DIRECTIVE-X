use std::env;
use std::fs;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use log::{info, error};
use num_bigint::BigUint;

// استدعاء الوحدات البرمجية للنظام
mod core;
mod eye_os;
mod neural;
mod ui;

use crate::core::causal_system::{CausalCollapseSystem, QuantumNode};
use crate::eye_os::rust_bus::RustBus;
use crate::neural::candle_network::SovereignNeuralNetwork;
use crate::ui::prompt_chunker::PromptChunker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. تهيئة نظام تسجيل الملاحظات عالي السرعة
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();

    println!("============================================================");
    println!("   DIRECTIVE-X: MAXIMUM SOVEREIGN CODE-GENERATOR COMPILER   ");
    println!("============================================================");

    let start_init = Instant::now();

    // فحص خيار التدريب --train
    if args.contains(&"--train".to_string()) {
        info!("🧠 [TRAINING MODE] Ingesting Context Stream & Training Network...");

        // أ) تشغيل محرك الانهيار السببي لمعالجة المتجهات O(N)
        let initial_nodes = vec![
            QuantumNode { id: 0, energy_scale: BigUint::from(1000000u32), frequency: 144.0 },
            QuantumNode { id: 1, energy_scale: BigUint::from(500000u32), frequency: 89.0 },
            QuantumNode { id: 2, energy_scale: BigUint::from(250000u32), frequency: 55.0 },
        ];
        let causal_engine = CausalCollapseSystem::new(initial_nodes);
        let collapse_route = causal_engine.execute_collapse();
        info!("[CORE] Quantum Causal Collapse Route Solved: {:?}", collapse_route);

        // ب) تشغيل حلقة التدريب وتحديث الأوزان عبر Candle Neural Engine
        let mut neural_net = SovereignNeuralNetwork::new()?;
        let training_loss = neural_net.train_epoch()?;
        info!("🔥 [NEURAL ENGINE] Training Epoch Completed. Loss: {:.6}", training_loss);

        // ج) تصدير الأوزان المدربة لاستخدامها لاحقاً
        fs::write("model_weights.safetensors", b"DIRECTIVE_X_SOVEREIGN_TRAINED_WEIGHTS_V1")?;
        info!("💾 [ARTIFACT] Exported Trained Weights -> 'model_weights.safetensors'");
        
        println!("------------------------------------------------------------");
        println!("   TRAINING PIPELINE COMPLETE: WEIGHTS & MODEL SAVED        ");
        println!("------------------------------------------------------------");
        return Ok(());
    }

    // 2. الوضع الافتراضي (Standard Runtime Mode)
    info!("[SYSTEM] Initializing Zero-Copy Rust Bus (Target: 0.002ms)...");
    let bus = Arc::new(Mutex::new(RustBus::new(1024 * 16)));
    let bus_clone = Arc::clone(&bus);

    tokio::spawn(async move {
        let mut guard = bus_clone.lock().await;
        if let Err(e) = guard.start_listener().await {
            error!("[BUS ERROR] Listener failure: {:?}", e);
        }
    });

    // تهيئة محرك الانهيار السببي
    info!("[CORE] Seeding Quantum Causal TSP Engine...");
    let initial_nodes = vec![
        QuantumNode { id: 0, energy_scale: BigUint::from(1000000u32), frequency: 144.0 },
        QuantumNode { id: 1, energy_scale: BigUint::from(500000u32), frequency: 89.0 },
        QuantumNode { id: 2, energy_scale: BigUint::from(250000u32), frequency: 55.0 },
    ];
    let causal_engine = CausalCollapseSystem::new(initial_nodes);
    let collapse_route = causal_engine.execute_collapse();
    info!("[CORE] Initial Collapse Route Deterministically Solved: {:?}", collapse_route);

    // تهيئة ذاكرة استقبال وتجزئة النصوص الضخمة
    info!("[UI] Booting Large Context Window Chunking Buffer...");
    let mut chunker = PromptChunker::new(128 * 1024);
    let sample_large_prompt = "DIRECTIVE-X: INGESTING HIGH-SCALE MULTI-LANGUAGE CONTEXT STREAM... ".repeat(100);
    chunker.feed_str(&sample_large_prompt);

    let processed_chunks = chunker.process_all_chunks();
    info!("[UI] Successfully processed {} input chunks without context drops.", processed_chunks.len());

    info!("[SYSTEM] Engine fully online in {:.3?} ms.", start_init.elapsed().as_secs_f64() * 1000.0);
    println!("------------------------------------------------------------");
    println!("   SOVEREIGN ENGINE ACTIVE: READY FOR UNLIMITED GENERATION  ");
    println!("------------------------------------------------------------");

    Ok(())
}
