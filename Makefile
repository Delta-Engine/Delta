# Delta Programming Language Makefile

# Build the compiler
build:
	cargo build --release

# Build for development
dev:
	cargo build

# Test interpreter mode
test-interpret:
	./target/debug/delta examples/compile_test.de --interpret

# Test compilation mode
test-compile:
	./target/debug/delta examples/compile_test.de --compile

# Run all examples in interpreter mode
run-examples:
	@echo "=== Simple Example ==="
	./target/debug/delta examples/simple.de --interpret
	@echo "\n=== Conditionals Example ==="
	./target/debug/delta examples/conditionals.de --interpret
	@echo "\n=== Arithmetic Example ==="
	./target/debug/delta examples/arithmetic.de --interpret

# Clean build artifacts
clean:
	cargo clean

# Help
help:
	@echo "Delta Programming Language - Build Commands"
	@echo ""
	@echo "build           - Build release version"
	@echo "dev             - Build development version"
	@echo "test-interpret  - Test interpreter mode"
	@echo "test-compile    - Test compilation to LLVM IR"
	@echo "compile-to-exe  - Compile Delta to native executable"
	@echo "run-examples    - Run all examples in interpreter mode"
	@echo "clean           - Clean build artifacts"
	@echo "help            - Show this help"

.PHONY: build dev test-interpret test-compile compile-to-exe run-examples clean help