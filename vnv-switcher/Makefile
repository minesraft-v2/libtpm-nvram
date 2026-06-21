CARGO := cargo
TARGET_DIR := target/release
BINARY_NAME := vnv-switcher
SCRIPT_PATH := scripts/menu.sh

.PHONY: all build clean run

all: build

build:
	@echo "[*] Triggering low-level compilation pipeline via Cargo..."
	$(CARGO) build --release
	@echo "[SUCCESS] Subsystem binary compiled accurately at $(TARGET_DIR)/$(BINARY_NAME)"

clean:
	@echo "[*] Flushing build environment artifacts..."
	$(CARGO) clean
	@echo "[SUCCESS] Workspace cleaned completely."

run: build
	@chmod +x $(SCRIPT_PATH)
	@./$(SCRIPT_PATH)
