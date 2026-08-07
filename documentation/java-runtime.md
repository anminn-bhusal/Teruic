
# Teruic OS — Embedded Java Runtime (Planned)

A core vision of Teruic OS is supporting a native, lightweight Java Bytecode Interpreter built directly inside the Rust kernel (`#![no_std]`).

## Architecture Goals

1. **Class File Parser:** Reads compiled `.class` binary streams from the VFS (`vfs.rs`).
2. **Class File Structure:**
   - Magic Header (`0xCAFEBABE`)
   - Constant Pool resolution
   - Method Table & Bytecode array extraction
3. **Interpreter Loop:** Reads JVM opcodes sequentially and updates an internal execution frame stack:
   - `0x10` (`bipush`): Push byte onto operand stack.
   - `0x60` (`iadd`): Add top two integers on stack.
   - `0xB6` (`invokevirtual`): Route system print calls to `vga.rs`.

## Contributor Integration

Java runtime engineers will work primarily in a new kernel module (`src/java_vm/`) that bridges file reads through `vfs.rs` and terminal output through `vga.rs`. It can be done once after the java_vm is built.