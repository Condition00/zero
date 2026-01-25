use x86_64::registers::model_specific::{LStar, SFMask, Star};
use x86_64::registers::rflags::RFlags;
use x86_64::VirtAddr;

//kernel stack for he syscalls
const SYSCALL_STACK_SIZE: usize = 4096 * 5;

#[repr(align(16))]
struct SyscallStack {
    data: [u8; SYSCALL_STACK_SIZE],
}

static mut SYSCALL_STACK: SyscallStack = SyscallStack {
    data: [0; SYSCALL_STACK_SIZE],
};

//syscall support

pub fn init() {
    //kernel stack for syscalls
    unsafe {
        let stack_top = VirtAddr::from_ptr(&raw const SYSCALL_STACK.data) + SYSCALL_STACK_SIZE;
        KERNEL_RSP = stack_top.as_u64();
    }

    LStar::write(VirtAddr::new(syscall_entry as u64));
    // Set segment selectors for syscall/sysret
    // Lower 32 bits: kernel CS/SS for syscall
    // Upper 32 bits: user CS/SS for sysret
    Star::write(
        crate::arch::x86_64::gdt::user_code_selector(),
        crate::arch::x86_64::gdt::user_data_selector(),
        crate::arch::x86_64::gdt::selectors().kernel_code_selector,
        crate::arch::x86_64::gdt::selectors().kernel_code_selector, // Kernel SS = CS for x64
    )
    .expect("Failed to write STAR MSR");

    //masking interrupts during syscall
    SFMask::write(RFlags::INTERRUPT_FLAG);
}

//assembly syscall entry point
#[unsafe(naked)]
extern "C" fn syscall_entry() {
    core::arch::naked_asm!(
        // Save user stack pointer
            "mov [rip + USER_RSP], rsp",

            // Switch to kernel stack (we'll need to set this up)
            "mov rsp, [rip + KERNEL_RSP]",

            // Save user registers that we need to preserve
            "push rcx",          // User RIP (saved by CPU)
            "push r11",          // User RFLAGS (saved by CPU)
            "push rbp",
            "push rbx",
            "push r12",
            "push r13",
            "push r14",
            "push r15",

            // Arguments are already in correct registers:
            // rax = syscall number
            // rdi = arg1, rsi = arg2, rdx = arg3
            // Call the Rust syscall handler
            "call {handler}",
            // Return value is in rax

            // Restore user registers
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbx",
            "pop rbp",
            "pop r11",          // User RFLAGS
            "pop rcx",          // User RIP

            // Restore user stack
            "mov rsp, [rip + USER_RSP]",

            // Return to userspace
            "sysretq",

            handler = sym syscall_handler,
    );
}

// Storage for stack pointers during syscall
static mut USER_RSP: u64 = 0;
static mut KERNEL_RSP: u64 = 0;

// Rust syscall handler
extern "C" fn syscall_handler(syscall_number: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    // We'll implement this next
    crate::println!(
        "[SYSCALL] Number: {}, Args: {}, {}, {}",
        syscall_number,
        arg1,
        arg2,
        arg3
    );
    0 // Return value
}
