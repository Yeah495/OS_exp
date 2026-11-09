#![no_std]
#![no_main]
mod console;

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const SYSCALL_EXIT: usize = 93;

//通用系统调用
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut a0 = args[0];

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
            options(nostack)
        );
    }

    a0 as isize
}

//输出系统调用
const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SYSCALL_WRITE,
        [fd, buffer.as_ptr() as usize, buffer.len()]
    )
}

//退出程序
pub fn sys_exit(code: i32) -> ! {
    syscall(SYSCALL_EXIT, [code as usize, 0, 0]);

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    print!("Hello, world! no ln\n");

    sys_exit(9)
}