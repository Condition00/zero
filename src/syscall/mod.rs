pub mod handlers;

use crate::println;
use x86_64::structures::idt::InterruptStackFrame;

// Syscall numbers (Linux-compatible)
pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_EXIT: usize = 60;

#[derive(Debug)]
#[repr(C)]
pub struct SyscallFrame {
    pub rax: u64, // syscall number
    pub rdi: u64, // arg 1
    pub rsi: u64, // arg 2
    pub rdx: u64, // arg 3
    pub r10: u64, // arg 4
    pub r8: u64,  // arg 5
    pub r9: u64,  // arg 6
}

pub extern "x86-interrupt" fn syscall_handler(mut stack_frame: InterruptStackFrame) {
    // In a real implementation, you'd extract registers from the stack
    // For now, we'll use inline assembly to read them

    let syscall_num: u64;
    let arg1: u64;
    let arg2: u64;
    let arg3: u64;

    unsafe {
        core::arch::asm!(
            "mov {}, rax",
            "mov {}, rdi",
            "mov {}, rsi",
            "mov {}, rdx",
            out(reg) syscall_num,
            out(reg) arg1,
            out(reg) arg2,
            out(reg) arg3,
        );
    }

    let result = match syscall_num as usize {
        SYS_READ => handlers::sys_read(arg1 as usize, arg2 as *mut u8, arg3 as usize),
        SYS_WRITE => handlers::sys_write(arg1 as usize, arg2 as *const u8, arg3 as usize),
        SYS_EXIT => handlers::sys_exit(arg1 as i32),
        _ => {
            println!("Unknown syscall: {}", syscall_num);
            Err(())
        }
    };

    // Return value in RAX (would need to modify stack frame)
    match result {
        Ok(val) => {
            // Set RAX in stack frame to return value
            // This requires more sophisticated stack manipulation
        }
        Err(_) => {
            // Return error code
        }
    }
}
