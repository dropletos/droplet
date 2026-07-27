#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(droplet::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use droplet::allocator::HEAP_SIZE;
use x86_64::VirtAddr;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use droplet::allocator;
    use droplet::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    droplet::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    let heap_val_1 = Box::new(41);
    let heap_val_2 = Box::new(13);
    assert_eq!(*heap_val_1, 41);
    assert_eq!(*heap_val_2, 13)
}

use alloc::vec::Vec;

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    droplet::test_panic_handler(info)
}
