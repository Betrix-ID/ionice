TARGET = arm64
SRC = src/main.rs
CARGO = cargo

# Default target
all: banner build

# Build the project
build:
	@echo ">> Compiling Rust program..."
	$(CARGO) build --release
	@echo ""
	@echo ">> Build finished successfully!"

# Run the compiled program
run:
	@echo ">> Running the program..."
	./target/release/$(TARGET)

# Banner tampilan keren
banner:
	@echo
	@echo
	@echo "     ☆=====================================☆"
	@echo
	@echo "      ~ Description Compile Rust Program"
	@echo
	@echo "      - Author             :  @UnixeID"
	@echo "      - Version            :  1.0"
	@echo "      - Release            :  05 - Mei - 2025"
	@echo "      - Project Name       :  Rust Compiler"
	@echo
	@echo "    |_______________________________________|"
	@echo "     \\______________________________________/"
	@echo
	@echo
	@sleep 1

# Clean build files
clean:
	@echo ">> Cleaning build files..."
	@rm -f Cargo.lock  
	@find target/release -type f ! -name "$(TARGET)" -exec rm -f {} +
	@find target/debug -type f ! -name "$(TARGET)" -exec rm -f {} +
	@find target -type d -empty -exec rmdir {} +
	@echo ">> Cleaned build files."

# Format Rust source code
format:
	@echo ">> Formatting Rust source code..."
	$(CARGO) fmt
	@echo ">> Source code formatted."

# Update dependencies if needed
update:
	@echo ">> Updating dependencies..."
	$(CARGO) update
	@echo ">> Dependencies updated."