#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rsos::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rsos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rsos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rsos::hlt_loop();
}

// Panic handlers for test and not test environments
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rsos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rsos::testing::test_panic_handler(info)
}
