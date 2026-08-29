# Directive-X Master Build Automation Engine

TARGET_DIR = target
BINARY_NAME = directive_x

.PHONY: all clean check build-linux build-windows build-macos build-android bench train

all: check build-linux

# التحقق البيئي الشامل من حزم الأدوات
check:
	@echo "[+] Checking Toolchains and Dependencies..."
	@cargo --version
	@rustup target list | grep installed

# بناء نسخة لينكس الأصلية فائقة التحسين
build-linux:
	@echo "[+] Compiling Native Ultra-Optimized Linux Binary..."
	cargo build --release --target x86_64-unknown-linux-gnu
	@echo "[✓] Linux Binary generated at $(TARGET_DIR)/x86_64-unknown-linux-gnu/release/$(BINARY_NAME)"

# تصريف نسخة الويندوز الرسمية المطابقة للمعمارية (تم تصحيحها لـ MSVC)
build-windows:
	@echo "[+] Cross-Compiling Windows Target (.exe) via MSVC..."
	cargo build --release --target x86_64-pc-windows-msvc
	@echo "[✓] Windows Executable generated at $(TARGET_DIR)/x86_64-pc-windows-msvc/release/$(BINARY_NAME).exe"

# تصريف نسخ الماكنتوش بالتوازي
build-macos:
	@echo "[+] Cross-Compiling macOS Target..."
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	@echo "[✓] macOS Binaries generated."

# هندسة بناء وتجهيز مكتبات الهواتف الذكية الحديثة بأعلى كفاءة طاقة (ARM64)
build-android:
	@echo "[+] Cross-Compiling Android NDK Targets (aarch64-linux-android)..."
	cargo build --release --target aarch64-linux-android
	@echo "[✓] Pure ARM64 Android NDK Binary generated successfully."

# أمر التدريب الفوري والشامل لابتلاع كافة مجموعات البيانات الحية محلياً
train:
	@echo "[+] Executing Sovereign Multi-Dataset Ingestion Engine..."
	cargo run --release -- --train

# اختبار الأداء الدقيق والقياس الحاد للخوارزمية
bench:
	@echo "[+] Executing Criterion High-Precision Performance Benchmarks..."
	cargo bench

# إبادة ملفات البناء المؤقتة لتنظيف الذاكرة
clean:
	@echo "[+] Cleaning target Directory..."
	cargo clean
