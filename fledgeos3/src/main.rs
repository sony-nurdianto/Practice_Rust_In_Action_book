#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    White = 0xF,
    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    Yellow = 0xE,
    DarkGray = 0x8,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;

        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();

        let framebuffer = 0xb8000 as *mut u8;
        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2
        }
    }
}

#[allow(unused_unsafe)]
#[panic_handler]
#[unsafe(no_mangle)]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        intrinsics::abort();
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let text = b"Rust in Action";

    let mut cursor = Cursor {
        position: 0,
        background: Color::Black,
        foreground: Color::BrightGreen,
    };

    cursor.print(text);

    loop {}
}
