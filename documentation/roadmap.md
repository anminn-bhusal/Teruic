# Teruic OS — Development Roadmap

## Completed Milestones
- [x] Bare-metal `x86_64` bootloader process.
- [x] Setup of the memory allocator and the custom kernel heap.
- [x] A driver is provided for the memory-mapped VGA terminal with a 80x25 text buffer located at `0xb8000`.
- [x] The in-memory Virtual File System (Ramdisk) is thread-safe.
- The shell terminal environment has script execution enabled (`exec script.sh`).
- [x] The processing of hardware interrupts (including the PIT Timer ticks and PS/2 Keyboard scancodes).
- [x] A foreign function interface capable of executing code in the C language.


## Active Milestone: VFS Interactive Text Editor (`edit`/`nano`)

The aim is to create a multi-line, terminal-based text editor which allows you to view and edit VFS files in real time.

- [ ] Include the function for repositioning the hardware cursor (`set_cursor(x, y)`) in the file `vga.rs`.
- [ ] Capture the original keyboard inputs directly when in editor mode (for the `Arrow Keys`, `Backspace`, and `Enter`).
- [ ] Make a data structure for a text buffer that can handle more than one line.
- [ ] Implement the shortcut handlers: Ctrl+S for Save to VFS path, and Ctrl+X for Exit to Shell.


## Upcoming Milestones
- [ ] A process scheduler that allows multiple tasks to be carried out.
- [ ] Built-in Java `.class` bytecode interpreter (`java-runtime.md`).
IDE/ATA hard disk driver for non-volatile storage.