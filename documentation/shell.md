# Teruic OS — Interactive Shell & Commands

The shell (src/shell.rs) is the main terminal interface and operates as an asynchronous kernel task; it takes in keyboard input in the form of scancodes, interprets command strings, keeps track of the execution history and is capable of running .sh batch scripts.

## Built-In Commands

| Command | Syntax | Description |
| :--- | :--- | :--- |
| ls | ls [path] | Lists the files and directories in the VFS ramdisk. |
| cat | cat <file> | Prints the byte contents of the file as ASCII text to the screen. |
| write | write <file> <text> | This command creates or overwrites a VFS file with the given string. |
| `exec` | `exec <script.sh>` | Runs the lines from a `.sh` file one after another. |
The C binary modules are caused to have their test execution carried out through the use of the FFI (c_abi.rs).
| info | info | Shows the CPU, architecture, and memory statistics. |
| uptime | uptime | Determines the number of system ticks to display how long the system has been running in seconds. |

Batch scripting (using the .sh runner)

The shell allows for the execution of scripts over several lines. A .sh file located within the VFS that contains a sequence of commands can be executed directly:

```text
# Example script stored at /demo.sh
write /hello.txt Hello_Teruic_OS
cat /hello.txt
uptime
```

To run the script:

```Bash
Teruic> exec /demo.sh
```


### File 3: `documentation/java-runtime.md`

```markdown
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