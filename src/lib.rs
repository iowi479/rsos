#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod serial;
pub mod testing;
pub mod vga_buffer;

/// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

pub fn init() {
    interrupts::init_idt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::testing::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception and if we continue after this interrupt, we succeed
    x86_64::instructions::interrupts::int3();
}
