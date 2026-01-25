//userspace syscall wrapper library to avoid writing assembly </3

#![no_std]

use core::arch::asm;

pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_STAT: u64 = 4;
pub const SYS_READDIR: u64 = 5;
pub const SYS_MKDIR: u64 = 6;
pub const SYS_TOUCH: u64 = 7;
pub const SYS_RM: u64 = 8;
pub const SYS_CLEAR: u64 = 9;
pub const SYS_REBOOT: u64 = 10;
pub const SYS_EXIT: u64 = 11;
pub const SYS_YIELD: u64 = 12;
