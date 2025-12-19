# Delta Programming Language Makefile

# Build the compiler
build-release:
	cargo build --release

# Build for development
build-dev:
	cargo build

# Test interpreter mode (Needs Dev Build)
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
	@echo ""
	@echo "Delta Programming Language - Build Commands"
	@echo "By PranavVerma-droid."
	@echo "Usage: make <COMMAND>"
	@echo ""
	@echo "	build-release   - Build release version"
	@echo "	build-dev       - Build development version"
	@echo "	test-interpret  - Test interpreter mode (Requires Dev Build)"
	@echo "	test-compile    - Test compilation to LLVM IR (Requires Dev Build)"
	@echo "	run-examples    - Run all examples in interpreter mode"
	@echo "	clean           - Clean build artifacts"
	@echo "	help            - Show this help menu"
	@echo ""

.PHONY: build-release build-dev test-interpret test-compile run-examples clean help