/// Simple userspace test program
///
/// Disassembly:
/// ```asm
/// 0x00: mov rax, 1          ; SYS_WRITE (7 bytes)
/// 0x07: mov rdi, 1          ; stdout (7 bytes)
/// 0x0E: lea rsi, [rip+0x19] ; message pointer (7 bytes) - RIP=0x15 after, msg at 0x2E
/// 0x15: mov rdx, 22         ; message length (7 bytes)
/// 0x1C: int 0x80            ; syscall (2 bytes)
/// 0x1E: mov rax, 60         ; SYS_EXIT (7 bytes)
/// 0x25: mov rdi, 0          ; exit code (7 bytes)
/// 0x2C: int 0x80            ; syscall (2 bytes)
/// 0x2E: msg: "Hello from userspace!\n" (22 bytes)
/// ```
pub fn get_test_program() -> &'static [u8] {
    &[
        // mov rax, 1 (SYS_WRITE)
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00,
        // mov rdi, 1 (stdout)
        0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00,
        // lea rsi, [rip + 0x19] - from 0x15 (next instr) to 0x2E (message)
        0x48, 0x8d, 0x35, 0x19, 0x00, 0x00, 0x00,
        // mov rdx, 22 (message length: "Hello from userspace!\n")
        0x48, 0xc7, 0xc2, 0x16, 0x00, 0x00, 0x00,
        // int 0x80 (syscall)
        0xcd, 0x80,
        // mov rax, 60 (SYS_EXIT)
        0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00,
        // mov rdi, 0 (exit code)
        0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00,
        // int 0x80 (syscall)
        0xcd, 0x80,
        // Message: "Hello from userspace!\n" (22 bytes)
        b'H', b'e', b'l', b'l', b'o', b' ',
        b'f', b'r', b'o', b'm', b' ',
        b'u', b's', b'e', b'r', b's', b'p', b'a', b'c', b'e',
        b'!', b'\n',
    ]
}
