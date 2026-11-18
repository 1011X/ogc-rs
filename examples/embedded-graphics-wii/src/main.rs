#![no_std]
#![no_main]

mod display;
use crate::display::Display;

extern crate alloc;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, RgbColor, Size, Transform},
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};

use ogc_rs::prelude::*;
use ogc_rs::{gx, video};

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    video::init();
    
    let config = RenderConfig::default();
    let mut fb = System::allocate_framebuffer(&config);
    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);
    wii_ctrl
        .as_wpad()
        .set_data_format(WPadDataFormat::ButtonsAccelIR);

    //Console::init(fb, &config);
    video::configure(&config);
    video::set_next_framebuffer(&fb);
    video::set_black(true);
    video::flush();
    video::wait_vsync();

    let mut wii_display = Display::new(256 * 1024);
    wii_display.setup(&config);
    video::set_black(false);

    println!("Hello World!");

    const BACKGROUND: Rectangle = Rectangle::new(Point::zero(), Size::new(640, 528));

    const CENTER: Point = Point::new(640 / 2, 528 / 2);
    let area: Rectangle = Rectangle::with_center(CENTER, Size::new(200, 200));

    const POINTER: Rectangle = Rectangle::new(Point::zero(), Size::new_equal(10));
    loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);

        if gcn_ctrl.is_button_down(Button::Start) {
            break 0;
        }

        if wii_ctrl.is_button_down(Button::Home) {
            break 0;
        }
        gx::set_viewport(
            0.0,
            0.0,
            config.fb_width as f32,
            config.efb_height as f32,
            0.0,
            0.0,
        );

        let ir = Point::new(
            wii_ctrl.as_wpad().ir().0 as i32,
            wii_ctrl.as_wpad().ir().1 as i32,
        );

        BACKGROUND
            .into_styled(PrimitiveStyle::with_fill(Rgb888::WHITE))
            .draw(&mut wii_display)
            .unwrap();

        if POINTER.translate(ir).intersection(&area).size != Size::zero() {
            area.into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                .draw(&mut wii_display)
                .unwrap();
            if wii_ctrl.is_button_held(Button::A) {
                area.into_styled(PrimitiveStyle::with_fill(Rgb888::GREEN))
                    .draw(&mut wii_display)
                    .unwrap();
            }
        } else {
            area.into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
                .draw(&mut wii_display)
                .unwrap();
        }

        POINTER
            .translate(ir)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::CYAN))
            .draw(&mut wii_display)
            .unwrap();

        wii_display.flush(&mut fb);

        video::set_next_framebuffer(&fb);
        video::flush();
        video::wait_vsync();
    }
}
