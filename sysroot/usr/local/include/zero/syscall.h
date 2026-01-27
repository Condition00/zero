#ifndef _ZERO_SYSCALL_H
#define _ZERO_SYSCALL_H

// Syscall numbers
#define SYS_READ 0
#define SYS_WRITE 1
#define SYS_OPEN 2
#define SYS_CLOSE 3
#define SYS_STAT 4
#define SYS_READDIR 5
#define SYS_MKDIR 6
#define SYS_TOUCH 7
#define SYS_RM 8
#define SYS_CLEAR 9
#define SYS_REBOOT 10
#define SYS_EXIT 11
#define SYS_YIELD 12

// Additional syscalls mlibc needs (have to implement)
#define SYS_MMAP 13
#define SYS_MUNMAP 14
#define SYS_SEEK 15
#define SYS_GETCWD 16
#define SYS_CHDIR 17
#define SYS_GETPID 18
#define SYS_CLOCK_GET 19
#define SYS_FSTAT 20
#define SYS_IOCTL 21

// Inline syscall wrappers using x86_64 syscall instruction
static inline long __syscall0(long n) {
	long ret;
	__asm__ volatile("syscall" : "=a"(ret) : "a"(n) : "rcx", "r11", "memory");
	return ret;
}

static inline long __syscall1(long n, long a1) {
	long ret;
	__asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1) : "rcx", "r11", "memory");
	return ret;
}

static inline long __syscall2(long n, long a1, long a2) {
	long ret;
	__asm__ volatile("syscall" : "=a"(ret) : "a"(n), "D"(a1), "S"(a2) : "rcx", "r11", "memory");
	return ret;
}

static inline long __syscall3(long n, long a1, long a2, long a3) {
	long ret;
	__asm__ volatile("syscall"
	                 : "=a"(ret)
	                 : "a"(n), "D"(a1), "S"(a2), "d"(a3)
	                 : "rcx", "r11", "memory");
	return ret;
}

static inline long __syscall4(long n, long a1, long a2, long a3, long a4) {
	long ret;
	register long r10 __asm__("r10") = a4;
	__asm__ volatile("syscall"
	                 : "=a"(ret)
	                 : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10)
	                 : "rcx", "r11", "memory");
	return ret;
}

static inline long __syscall5(long n, long a1, long a2, long a3, long a4, long a5) {
	long ret;
	register long r10 __asm__("r10") = a4;
	register long r8 __asm__("r8") = a5;
	__asm__ volatile("syscall"
	                 : "=a"(ret)
	                 : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10), "r"(r8)
	                 : "rcx", "r11", "memory");
	return ret;
}

static inline long __syscall6(long n, long a1, long a2, long a3, long a4, long a5, long a6) {
	long ret;
	register long r10 __asm__("r10") = a4;
	register long r8 __asm__("r8") = a5;
	register long r9 __asm__("r9") = a6;
	__asm__ volatile("syscall"
	                 : "=a"(ret)
	                 : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10), "r"(r8), "r"(r9)
	                 : "rcx", "r11", "memory");
	return ret;
}

#endif // _ZERO_SYSCALL_H
