#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zero::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use zero::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    //invoking a breakpoint function
    zero::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    //trigering the page fault to check
    unsafe {
        *(0xdeadbeef as *mut u8) = 67;
    };

    println!("It did not crash");
    loop {}
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zero::test_panic_handler(info)
}
