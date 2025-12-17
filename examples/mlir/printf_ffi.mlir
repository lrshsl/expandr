module {
  // 1. Declare the external printf function
  // signature: int printf(char *format, ...)
  // !llvm.ptr is the opaque pointer type (void*)
  llvm.func @printf(!llvm.ptr, ...) -> i32

  // 2. Define a global string constant
  // We include \0A (newline) and \00 (null terminator) explicitly.
  llvm.mlir.global internal constant @hello_str("Hello from MLIR!\0A\00") 
    {addr_space = 0 : i32}

  llvm.func @main() -> i32 {
    // 3. Get the address of the global string
    // This returns a pointer to the array [18 x i8]
    %0 = llvm.mlir.addressof @hello_str : !llvm.ptr

    // 4. Get a pointer to the first character (decay array to pointer)
    // In C this is equivalent to: char *ptr = &hello_str[0];
    %ptr = llvm.getelementptr %0[0, 0] 
      : (!llvm.ptr) -> !llvm.ptr, !llvm.array<18 x i8>

    // 5. Call printf
    // We must specify the full signature including varargs (...)
    %ret = llvm.call @printf(%ptr) vararg(!llvm.func<i32 (ptr, ...)>) 
      : (!llvm.ptr) -> i32

    // Return 0
    %exit_code = llvm.mlir.constant(0 : i32) : i32
    llvm.return %exit_code : i32
  }
}
