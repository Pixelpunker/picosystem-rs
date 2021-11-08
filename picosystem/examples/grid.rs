#![no_std]
#![no_main]

use cortex_m_rt::entry;
use log::info;
use picosystem::hardware;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;

#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let mut hw = hardware::Hardware::new();

    info!("Finished initialization");
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    loop {
        hw.display.draw(|display| {
            display.clear(Rgb565::BLACK).unwrap();
            for i in 0..240 {
                if i % 16 == 0 {
                    Line::new(Point::new(x + i, 0), Point::new(x + i, 239))
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
                        .draw(display)
                        .unwrap();
                }
                if i % 16 == 0 {
                    Line::new(Point::new(0, y + i), Point::new(239, y + i))
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 1))
                        .draw(display)
                        .unwrap();
                }
            }
            x = (x + 2) % 16;
            y = (y + 2) % 16;
        });
    }
}
