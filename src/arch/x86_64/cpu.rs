use x86_64::instructions::port::Port;

pub fn reboot() -> ! {
    unsafe {
        let mut port = Port::<u8>::new(0x64);
        port.write(0xFE);
    }

    loop {
        x86_64::instructions::hlt();
    }
}

pub fn shutdown() -> ! {
    unsafe {
        let mut port1 = Port::<u16>::new(0x604);
        port1.write(0x2000);

        let mut port = Port::<u16>::new(0xB004);
        port.write(0x2000u16);
    }

    loop {
        x86_64::instructions::hlt();
    }
}
