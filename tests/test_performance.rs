use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use num_bigint::BigUint;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

// استدعاء المكونات الأساسية للنظام للاختبار القياسي
#[path = "../src/core/mod.rs"]
mod core;
#[path = "../src/eye_os/mod.rs"]
mod eye_os;
#[path = "../src/ui/mod.rs"]
mod ui;

use crate::core::causal_system::{CausalCollapseSystem, QuantumNode};
use crate::eye_os::rust_bus::RustBus;
use crate::ui::prompt_chunker::PromptChunker;

/// اختبار كفاءة محرك الانهيار السببي الحتمي Causal Collapse O(N)
fn bench_causal_collapse_engine(c: &mut Criterion) {
    let mut group = c.benchmark_group("CausalCollapseSystem");

    for size in [100, 1_000, 10_000].iter() {
        let nodes: Vec<QuantumNode> = (0..*size)
            .map(|i| QuantumNode {
                id: i,
                energy_scale: BigUint::from((i * 1000 + 1) as u64),
                frequency: (i as f64 * 0.1) % 100.0 + 1.0,
            })
            .collect();

        let engine = CausalCollapseSystem::new(nodes);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("ExecuteCollapse", size), size, |b, _| {
            b.iter(|| {
                let _route = engine.execute_collapse();
            });
        });
    }
    group.finish();
}

/// اختبار زمن الاستجابة الفائق لناقل البيانات Zero-Copy Rust Bus (Target: <= 0.002ms)
fn bench_zero_copy_rust_bus(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("RustBus_Throughput");

    let payload = vec![0u8; 1024 * 64]; // 64 KB Payload
    let bus = Arc::new(Mutex::new(RustBus::new(2048)));

    group.throughput(Throughput::Bytes(payload.len() as u64));
    group.bench_function("Publish_64KB_ZeroCopyFrame", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus_guard = bus.lock().await;
                let _ = bus_guard.publish(1, &payload).await;
            });
        });
    });

    group.finish();
}

/// اختبار قدرة تجزئة واستيعاب النصوص والمدخلات الضخمة Prompt Chunker
fn bench_large_prompt_chunker(c: &mut Criterion) {
    let mut group = c.benchmark_group("PromptChunker_LargeStream");

    // محاكاة برومبت نصي عملاق بحجم 1 ميجابايت
    let massive_prompt = "DIRECTIVE_X_HIGH_SCALE_CONTEXT_TOKEN_BUFFER_STREAM ".repeat(20_000);

    group.throughput(Throughput::Bytes(massive_prompt.len() as u64));
    group.bench_function("Process_1MB_Chunker_Buffer", |b| {
        b.iter(|| {
            let mut chunker = PromptChunker::new(64 * 1024);
            chunker.feed_str(&massive_prompt);
            let _chunks = chunker.process_all_chunks();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_causal_collapse_engine,
    bench_zero_copy_rust_bus,
    bench_large_prompt_chunker
);
criterion_main!(benches);
