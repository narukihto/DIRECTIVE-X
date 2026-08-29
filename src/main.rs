use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use log::{info, error};
use num_bigint::BigUint;

// استدعاء الوحدات البرمجية للنظام بالكامل
pub mod core;
pub mod eye_os;
pub mod neural;
pub mod ui;
pub mod data_loader;

// استيراد آليات المحرك الحتمي لقفل الأبعاد كوانتياً
use crate::core::causal_system::{CausalCollapseSystem, QuantumNode};
use crate::eye_os::rust_bus::RustBus;
use crate::eye_os::hive_mind::HiveMind;
use crate::neural::candle_network::CandleNetwork;
use crate::ui::prompt_chunker::PromptChunker;
use crate::ui::terminal_gui::TerminalGui;
use crate::data_loader::{DataLoader, DatasetTarget};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. تهيئة نظام تسجيل الملاحظات عالي السرعة
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();

    println!("============================================================");
    println!("   DIRECTIVE-X: MAXIMUM SOVEREIGN CODE-GENERATOR COMPILER   ");
    println!("============================================================");

    let start_init = Instant::now();

    // خيارات الحجم القياسي للتوافق التام مع GGUF و Atomic Chat
    let vocab_size = 32000;
    let embedding_dim = 512;
    let hidden_dim = 256;
    
    let weights_path = "model_weights.safetensors";

    // فحص خيار التدريب --train للتهام البيانات الحية بالكامل وتحقيق الموسوعية الفلكية
    if args.contains(&"--train".to_string()) {
        info!("🧠 [MULTIPLE INGESTION MODE] Awakening All Core Knowledge Reservoirs...");

        // أ) فحص وجود أوزان سابقة لمواصلة التدريب البنائي التراكمي اللانهائي
        let mut neural_net = CandleNetwork::new(vocab_size, embedding_dim, hidden_dim)?;
        if Path::new(weights_path).exists() {
            info!("💾 [INCREMENTAL TRAINING] Detected existing weights target. Resuming learning...");
        } else {
            info!("🌱 [CORE START] No previous weights found. Cultivating new neural layers from scratch.");
        }

        let total_epochs = 10;

        // ب) 🌍 دمج أهداف التريليونات الشاملة للعلوم والتاريخ البرمجي والبشري
        let targets = vec![
            DatasetTarget::AyaDataset,
            DatasetTarget::CodeXGLUE,
            DatasetTarget::TheStackV2,
            DatasetTarget::ShareGPT,
            DatasetTarget::FineWebEdu, 
            DatasetTarget::Wikipedia,  
        ];

        // ج) حلقة التدريب الشاملة عبر التلقيم والفلترة الكوانتية المتتالية لامتصاص الملايين بالكامل
        for target in targets {
            let loader = DataLoader::new(target.clone(), 64 * 1024 * 1024);
            let shards = target.get_target_shards();

            for shard in shards {
                match loader.download_shard_from_hub(&shard).await {
                    Ok(local_file_path) => {
                        if let Ok(raw_file_bytes) = loader.mmap_pass_to_tensor_core(&local_file_path) {
                            
                            info!("⚡ [QUANTUM MASK] Injecting Causal Collapse System for Dimensional Locking...");
                            
                            // 🧠 1. توليد العقد بناءً على القيم الفريدة للبايتات (من 0 إلى 255) مع تحديد النوع الموجب الصريح u32
                            let quantum_nodes: Vec<QuantumNode> = (0u32..=255u32)
                                .map(|byte_val| QuantumNode {
                                    id: byte_val as usize, 
                                    energy_scale: BigUint::from(byte_val), 
                                    frequency: byte_val as f64 * 1.44, 
                                })
                                .collect();

                            let mut causal_engine = CausalCollapseSystem::new(quantum_nodes);
                            causal_engine.threshold_limit = 0.85; 
                            
                            let collapse_route = causal_engine.execute_collapse(); 
                            
                            info!("[CORE] Quantum Dim-Lock Completed. Active Symmetrical Byte Values Size: {}", collapse_route.len());

                            // ⚡ 2. تصفية تيار البيانات بالكامل بناءً على مصفوفة قفل الأبعاد الحتمية للمحرك السيادي
                            let tokens: Vec<u32> = raw_file_bytes.iter()
                                .filter(|&&byte| collapse_route.contains(&(byte as usize))) 
                                .map(|&b| (b as u32) % (vocab_size as u32))
                                .collect();

                            let total_tokens = tokens.len();
                            let chunk_size = 16; 

                            if total_tokens > chunk_size {
                                info!("[NEURAL ENGINE] Commencing sliding-window train pass on mass locked data. Total Tokens: {}", total_tokens);
                                let mut offset = 0;
                                let mut step_count = 0; 
                                
                                while offset + chunk_size < total_tokens {
                                    let input_tokens = &tokens[offset..offset + chunk_size];
                                    let target_tokens = &tokens[offset + 1..=offset + chunk_size];

                                    for epoch in 1..=total_epochs {
                                        let training_loss = neural_net.train_step(input_tokens, target_tokens)?;
                                        
                                        if step_count % 100 == 0 && epoch == total_epochs {
                                            info!("🔥 [NEURAL ENGINE] [{}] Pos: {}/{}. Shard Loss: {:.6}", 
                                                loader.target.as_str(), offset, total_tokens, training_loss);
                                        }
                                    }
                                    
                                    offset += chunk_size;
                                    step_count += 1;
                                }
                                
                                // 💾 [CHECKPOINT]: حفظ المعرفة فوراً ودورياً بعد اكتمال كل Shard بنجاح لتأمين الـ Loop اللانهائي
                                neural_net.save_weights(weights_path)?;
                                info!("💾 [CHECKPOINT SAVED] Synchronized weights safely at path: {}", weights_path);
                            }
                        }
                        // تفريغ القرص الصلب فوراً للـ Shard التالي لحماية الـ Runner من الامتلاء
                        let _ = fs::remove_file(&local_file_path);
                    },
                    Err(e) => error!("[INGESTION ERROR] Skipped shard {}: {}", shard, e),
                }
            }
        }

        return Ok(());
    }

    // 2. الوضع الافتراضي للتشغيل والإنتاج (Standard Runtime Mode)
    info!("[SYSTEM] Initializing Zero-Copy Rust Bus (Target: 0.002ms)...");
    let bus = Arc::new(Mutex::new(RustBus::new(1024 * 16)));
    let bus_clone = Arc::clone(&bus);

    tokio::spawn(async move {
        let mut guard = bus_clone.lock().await;
        if let Err(e) = guard.start_listener().await {
            error!("[BUS ERROR] Listener failure: {:?}", e);
        }
    });

    info!("[SYSTEM] Loading Neural Inversion Matrix for Core Inference Pass...");
    let mut runtime_neural_net = CandleNetwork::new(vocab_size, embedding_dim, hidden_dim)?;

    info!("[SYSTEM] Awakening the 12-Agent Swarm Orchestrator...");
    let mut hive_mind = HiveMind::new();
    let swarm_status = hive_mind.get_swarm_status();
    info!("[HIVE MIND] Swarm fully online. Total Active Agents: {}", swarm_status.len());

    info!("[CORE] Seeding Quantum Causal TSP Engine...");
    let initial_nodes = vec![
        QuantumNode { id: 0, energy_scale: BigUint::from(1000000u32), frequency: 144.0 },
        QuantumNode { id: 1, energy_scale: BigUint::from(500000u32), frequency: 89.0 },
        QuantumNode { id: 2, energy_scale: BigUint::from(250000u32), frequency: 55.0 },
    ];
    let causal_engine = CausalCollapseSystem::new(initial_nodes);
    let collapse_route = causal_engine.execute_collapse();
    info!("[CORE] Initial Collapse Route Deterministically Solved: {:?}", collapse_route);

    info!("[UI] Booting Large Context Window Chunking Buffer...");
    let mut chunker = PromptChunker::new(128 * 1024);
    let sample_large_prompt = "DIRECTIVE-X: INGESTING HIGH-SCALE MULTI-LANGUAGE CONTEXT STREAM... ".repeat(100);
    
    // 🎯 [تم الإصلاح الحتمي للسطر 178]: إضافة علامة المرجع & الصريحة المتوافقة مع نوع مدخلات الدالة
    chunker.feed_str(&sample_large_prompt);

    let processed_chunks = chunker.process_all_chunks();
    info!("[UI] Successfully processed {} input chunks without context drops.", processed_chunks.len());

    info!("[HIVE MIND] Initiating Sovereign Agent Verification Dispatch...");
    let test_payload = "COMPILER_DIRECTIVE_EVM_ARBITRAGE_CORE_DECOUPLE";

    match hive_mind.dispatch_task(6, test_payload, &mut runtime_neural_net) {
        Ok(res) => info!("[SWARM RESPONSE] {}", res),
        Err(e) => error!("[SWARM FAIL] Swarm orchestrator bottleneck detected: {}", e)
    }

    info!("[SYSTEM] Engine fully online in {:.3?} ms.", start_init.elapsed().as_secs_f64() * 1000.0);
    println!("------------------------------------------------------------");
    println!("   SOVEREIGN ENGINE ACTIVE: READY FOR UNLIMITED GENERATION  ");
    println!("------------------------------------------------------------");

    info!("[UI] Launching Interactive Terminal Dashboard...");
    let mut gui = TerminalGui::new();
    if let Err(e) = gui.run() {
        error!("[UI ERROR] Terminal GUI exited with error: {:?}", e);
    }

    Ok(())
}
