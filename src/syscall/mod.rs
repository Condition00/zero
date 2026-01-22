pub mod handlers;

use crate::serial_println;

pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_EXIT: usize = 60;

// Syscall registers saved before interrupt
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct SyscallRegisters {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbp: u64,
    rbx: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
}

#[unsafe(naked)]
pub extern "C" fn syscall_handler() {
    unsafe {
        core::arch::naked_asm!(
            // Save all registers in order (push goes high to low addresses)
            "push rax",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push rbx",
            "push rbp",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            
            // RSP now points to our SyscallRegisters struct
            "mov rdi, rsp",  // Pass pointer to registers as first argument
            
            "call {}",
            
            // RAX contains return value, don't restore it
            "add rsp, 8",    // Skip r15 on stack
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbp",
            "pop rbx",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "add rsp, 8",    // Skip the saved RAX (return value is in RAX already)
            
            "iretq",
            
            sym syscall_rust_handler,
        );
    }
}

#[no_mangle]
extern "C" fn syscall_rust_handler(regs: &SyscallRegisters) -> u64 {
    serial_println!(
        "[SYSCALL] num={}, args=({}, {}, {})",
        regs.rax,
        regs.rdi,
        regs.rsi,
        regs.rdx
    );

    let result = match regs.rax as usize {
        SYS_READ => handlers::sys_read(regs.rdi as usize, regs.rsi as *mut u8, regs.rdx as usize),
        SYS_WRITE => handlers::sys_write(regs.rdi as usize, regs.rsi as *const u8, regs.rdx as usize),
        SYS_EXIT => handlers::sys_exit(regs.rdi as i32),
        _ => {
            serial_println!("[SYSCALL] Unknown syscall: {}", regs.rax);
            Err(())
        }
    };

    match result {
        Ok(val) => {
            serial_println!("[SYSCALL] Returning: {}", val);
            val as u64
        }
        Err(_) => u64::MAX, // -1 as error
    }
}
