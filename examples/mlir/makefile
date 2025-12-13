
# expandr -> MLIR
%.mlir: %.exr
	cargo run -- expand $< -o $@

# MLIR -> LLVM IR
%.ll: %.mlir
	mlir-translate --mlir-to-llvmir $< -o $@

# LLVM IR -> binary
%.bin: %.ll
	clang $< -o $@

