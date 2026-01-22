use crate::gdt;
use crate::{println, serial_println};
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::VirtAddr;

const USER_STACK_SIZE: usize = 4096 * 20; //80kb

pub fn jump_to_userspace(entry_point: VirtAddr, user_stack: VirtAddr) -> ! {
    serial_println!("[USERSPACE] Jumping to ring3!");
    serial_println!("[USERSPACE] Entry: {:?}", entry_point);
    serial_println!("[USERSPACE] Entry: {:?}", user_stack);

    let code_selector = gdt::user_code_selector();
    let data_selector = gdt::user_data_selector();

    unsafe {
        core::arch::asm!(
                "cli",
                "mov ds, {0:x}",         // Set data segment
                "mov es, {0:x}",
                "mov fs, {0:x}",
                "mov gs, {0:x}",

                "push {0:r}",            // SS (stack segment)
                "push {1:r}",            // RSP (user stack pointer)
                "push 0x202",            // RFLAGS (enable interrupts bit)
                "push {2:r}",            // CS (code segment)
                "push {3:r}",            // RIP (entry point)

                "iretq",                 // Return to ring 3!

                in(reg) data_selector.0,
                in(reg) user_stack.as_u64(),
                in(reg) code_selector.0,
                in(reg) entry_point.as_u64(),
                options(noreturn)
        );
    }
}

pub fn allocate_user_stack(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<VirtAddr, &'static str> {
    let stack_start = VirtAddr::new(0x0000_7000_0000_0000); //arbitray
    let stack_end = stack_start + USER_STACK_SIZE;

    let flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

    for addr in (stack_start.as_u64()..stack_end.as_u64()).step_by(4096) {
        let page = Page::containing_address(VirtAddr::new(addr));
        let frame = frame_allocator
            .allocate_frame()
            .ok_or("Failed to allocate frame for user stack")?;

        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .map_err(|_| "Failed to map user stack")?
                .flush();
        }
    }

    serial_println!(
        "[USERSPACE] Allocated user stack: {:?} - {:?}",
        stack_start,
        stack_end
    );
    Ok(stack_end)
}

pub fn load_user_program(
    code: &[u8],
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<VirtAddr, &'static str> {
    let code_start = VirtAddr::new(0x0000_4000_0000_0000); // arbitrary userspace address
    let code_size = code.len();
    let pages_needed = (code_size + 4095) / 4096;

    let flags =
        PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE; // can remove later for security

    // Allocate and map pages
    for i in 0..pages_needed {
        let offset = (i * 4096) as u64;
        let addr = code_start + offset; // Uses Add<u64> trait implementation
        let page = Page::containing_address(addr);
        let frame = frame_allocator
            .allocate_frame()
            .ok_or("Failed to allocate frame for user code")?;

        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .map_err(|_| "Failed to map user code")?
                .flush();
        }
    }
    // Copy code to userspace
    unsafe {
        let dest = code_start.as_u64() as *mut u8;
        core::ptr::copy_nonoverlapping(code.as_ptr(), dest, code.len());
    }

    serial_println!("[USERSPACE] Loaded {} bytes at {:?}", code_size, code_start);
    Ok(code_start)
}
