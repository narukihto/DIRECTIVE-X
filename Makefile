# Directive-X Master Build Automation Engine

TARGET_DIR = target
BINARY_NAME = directive_x

.PHONY: all clean check build-linux build-windows build-macos build-android bench

all: check build-linux

# Environmental Verification
check:
	@echo "[+] Checking Toolchains and Dependencies..."
	@cargo --version
	@rustup target list | grep installed

# Native Linux ELF Build
build-linux:
	@echo "[+] Compiling Native Ultra-Optimized Linux Binary..."
	cargo build --release --target x86_64-unknown-linux-gnu
	@echo "[✓] Linux Binary generated at $(TARGET_DIR)/x86_64-unknown-linux-gnu/release/$(BINARY_NAME)"

# Windows Executable Cross-Compilation
build-windows:
	@echo "[+] Cross-Compiling Windows Target (.exe)..."
	cargo build --release --target x86_64-pc-windows-gnu
	@echo "[✓] Windows Executable generated at $(TARGET_DIR)/x86_64-pc-windows-gnu/release/$(BINARY_NAME).exe"

# macOS Binary Cross-Compilation
build-macos:
	@echo "[+] Cross-Compiling macOS Target..."
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	@echo "[✓] macOS Binaries generated."

# Android Native Library / APK Package Setup
build-android:
	@echo "[+] Cross-Compiling Android NDK Targets (aarch64-linux-android)..."
	cargo build --release --target aarch64-linux-android
	cargo build --release --target armv7-linux-androideabi
	@echo "[✓] Android NDK Binaries generated."

# Continuous Benchmark Harness (Criterion)
bench:
	@echo "[+] Executing Criterion High-Precision Performance Benchmarks..."
	cargo bench

# Clean Build Artifacts
clean:
	@echo "[+] Cleaning target Directory..."
	cargo clean
