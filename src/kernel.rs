#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
    ptr,
};

extern "C" {
    static mut __bss: u32;
    static __bss_end: u32;
    static __stack_top: u32;
}
struct Sbiret {
    error: u32,
    value: u32,
}

fn sbi_call(
    arg0: u32,
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    fid: u32,
    eid: u32,
) -> Sbiret {
    let error: u32;
    let value: u32;
    unsafe {
        asm!("ecall",
        inout("a0") arg0 => error,inout("a1") arg1 => value,
        in("a2") arg2,in("a3") arg3,in("a4") arg4,
        in("a5") arg5,in("a6") fid,in("a7") eid);
    }
    Sbiret { error, value }
}

fn putchar(ch: u8) {
    sbi_call(ch as u32, 0, 0, 0, 0, 0, 0, 1);
}

#[no_mangle]
fn kernel_main() {
    unsafe {
        let bss = ptr::addr_of_mut!(__bss);
        let bss_end = ptr::addr_of!(__bss_end);
        ptr::write_bytes(bss, 0, bss_end as usize - bss as usize);
    }
    let s = "\n\nHello World!\n";
    for c in s.chars() {
        putchar(c as u8);
    }
    loop {}
}

#[link_section = ".text.boot"]
#[naked]
#[no_mangle]
extern "C" fn boot() {
    unsafe {
        naked_asm!(
            "la sp, {stack_top}",
            "j kernel_main",
            stack_top = sym __stack_top);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
