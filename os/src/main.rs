#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

#[macro_use] 
extern crate bitflags;
extern crate alloc;
#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod sync;
mod config;
mod loader;
mod timer;
mod mm;
pub mod syscall;
pub mod trap;
pub mod task;


global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);

    }
}


#[no_mangle]
pub fn rust_main() -> ! {

    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("[kernel] back to world!");
    mm::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

