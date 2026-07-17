#![no_std] // no standard lib lmfaoo
#![no_main] // no rust entry points
#![feature(custom_test_frameworks)]
#![test_runner(droplet::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{fmt::Write, panic::PanicInfo};
use droplet::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("HALLO{}", 4.9 / 5.4);

    droplet::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();

    println!("no crashy!!!");
    droplet::hlt_loop();
    loop {
        use droplet::print;
        print!("-")
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
    droplet::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    droplet::test_panic_handler(info);
}

//https://os.phil-opp.com/hardware-interrupts/
