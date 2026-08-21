#![no_std] // no standard lib lmfaoo
#![no_main] // no rust entry points
#![feature(custom_test_frameworks)]
#![test_runner(droplet::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::{fmt::Write, panic::PanicInfo};
use droplet::{allocator, memory, println};
use x86_64::{
    PhysAddr,
    structures::paging::{Page, Translate},
};
extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use droplet::memory;
    use droplet::memory::BootInfoFrameAllocator;
    use x86_64::{VirtAddr, structures::paging::page};

    println!("hellow world{}", "wow");
    droplet::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    }

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initalisation failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(1);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} agora",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    println!("no crashy!!!");
    droplet::hlt_loop();
    // loop {
    //     use droplet::print;
    //     print!("-")
    // }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    droplet::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    droplet::test_panic_handler(info);
}
