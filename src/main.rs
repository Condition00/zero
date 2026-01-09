#![no_std]
#![no_main]

mod vg_buffer;
//#![feature(asm)]
//panic implementation

use core::panic::PanicInfo;

//this will be called on kernel panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//overwriting the os entry point

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from really really low{}", "!");
    panic!();
}
