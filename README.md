# DIRECTIVE-X

DIRECTIVE-X/
├── Cargo.toml
├── Makefile
├── build.rs
├── README.md
├── .github/
│   └── workflows/
│       └── production_compile.yml
├── src/
│   ├── main.rs
│   ├── data_loader.rs
│   ├── core/
│   │   ├── mod.rs
│   │   └── causal_system.rs
│   ├── eye_os/
│   │   ├── mod.rs
│   │   ├── hive_mind.rs
│   │   └── rust_bus.rs
│   ├── neural/
│   │   ├── mod.rs
│   │   ├── candle_network.rs
│   │   └── custom_loss.rs
│   └── ui/
│       ├── mod.rs
│       ├── prompt_chunker.rs
│       └── terminal_gui.rs
├── mojo/
│   └── compute.mojo
├── python_core/
│   ├── __init__.py
│   └── orchestrator.py
└── tests/
    ├── test_performance.rs
    └── test_fuzzing.rs
