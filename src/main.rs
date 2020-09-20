#![no_std] // dont link rust standard library
#![no_main] // disable rust-level entry point

mod vga_buffer;

extern crate rlibc;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ",some numbers: {} {}",
    //     42,
    //     1.3777
    // )
    // .unwrap();
    // // vga_buffer::print_something();
    panic!("_info: &PanicInfo");
    // println!("Hello World {}", 3.14);
    loop {}
}
