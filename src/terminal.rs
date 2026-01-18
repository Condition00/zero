use crate::vg_buffer::WRITER;
use core::fmt::Write;

pub fn write(s: &str) {
    WRITER.lock().write_str(s).unwrap();
}

pub fn write_char(c: char) {
    let mut buf = [0; 4];
    write(c.encode_utf8(&mut buf));
}

pub fn clear() {
    let mut writer = WRITER.lock();
    for _ in 0..25 {
        writer.write_str("\n").unwrap();
    }
}
