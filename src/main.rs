#![no_std] // no standard lib lmfaoo
#![no_main] // no rust entry points
#![feature(custom_test_frameworks)]
#![test_runner(droplet::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::{fmt::Write, panic::PanicInfo};
use droplet::{memory, println};
use x86_64::{
    PhysAddr,
    structures::paging::{Page, Translate},
};

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

// https://os.phil-opp.com/paging-implementation/#bootloader-support - aqui
