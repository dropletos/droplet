#![no_std]
#![no_main]

use core::panic::PanicInfo;
use droplet::{QEC, exit_Q, serial_print, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("le test no panik");
    exit_Q(QEC::Success);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_Q(QEC::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail....\t");
    assert_eq!(0, 1);
}
