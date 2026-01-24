use x86_64::VirtAddr;

const USER_STACK_SIZE: usize = 4096 * 4;

#[repr(align(16))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static mut USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

fn user_function() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn user_stack_top() -> VirtAddr {
    let start = VirtAddr::from_ptr(unsafe { &USER_STACK });
    start + USER_STACK_SIZE
}

pub fn get_user_function_addr() -> VirtAddr {
    VirtAddr::new(user_function as *const () as u64)
}

pub unsafe fn jump_to_userspace(_entry: VirtAddr, _user_stack_top: VirtAddr) -> ! {
    let selectors = crate::arch::x86_64::gdt::selectors();

    // Step 1: agh Convert segment selectors to u64
    let user_cs_value = selectors.user_code_selector.0 as u64;
    let user_ss_value = selectors.user_data_selector.0 as u64;

    // Step 2: Get function address as u64
    let user_rip_value = _entry.as_u64();

    // Step 3: Get stack pointer as u64
    let user_rsp_value = _user_stack_top.as_u64();

    // Step 4: Execute the jump via iretq
    core::arch::asm!(
        "push {user_ss}",
        "push {user_rsp}",
        "push {user_rflags}",
        "push {user_cs}",
        "push {user_rip}",
        "iretq",
        user_ss = in(reg) user_ss_value,
        user_rsp = in(reg) user_rsp_value,
        user_rflags = in(reg) 0x200u64,
        user_cs = in(reg) user_cs_value,
        user_rip = in(reg) user_rip_value,
        options(noreturn)
    );
}
