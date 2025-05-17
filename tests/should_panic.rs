#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rsos::{
    serial_print, serial_println,
    testing::exiter::{QemuExitCode, exit_qemu},
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("should_panic::should_panic...\t");
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_panic() {
    panic!("should panic");
}
