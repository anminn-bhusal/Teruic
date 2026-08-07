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