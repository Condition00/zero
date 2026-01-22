use crate::print;
use crate::println;
use crate::serial_println;

pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> Result<usize, ()> {
    //validate if the pinter is in the userspace
    if buf.is_null() {
        return Err(());
    }

    //TODO: Proper memory validation

    if fd == 1 || fd == 2 {
        // stdout or stderr
        let slice = unsafe { core::slice::from_raw_parts(buf, count) };

        if let Ok(s) = core::str::from_utf8(slice) {
            print!("{}", s);
            serial_println!("[SYSCALL] write: {}", s);
            Ok(count)
        } else {
            // Raw bytes
            for &byte in slice {
                print!("{}", byte as char);
            }
            Ok(count)
        }
    } else {
        Err(())
    }
}

pub fn sys_read(_fd: usize, _buf: *mut u8, _count: usize) -> Result<usize, ()> {
    // TODO: Implement
    Err(())
}

pub fn sys_exit(code: i32) -> Result<usize, ()> {
    println!("Process exited with code: {}", code);
    serial_println!("[SYSCALL] exit: {}", code);

    // TODO: Actually terminate the process and schedule next one
    // For now, just halt since we're in test mode before executor runs
    loop {
        x86_64::instructions::hlt();
    }
}
