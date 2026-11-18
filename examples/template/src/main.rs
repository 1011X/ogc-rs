#![no_std]
#![no_main]

extern crate alloc;

use ogc_rs::{mp3player::MP3Player, prelude::*, video};

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mp3 = include_bytes!("../mp3.mp3");

    video::init();
    let config = RenderConfig::default();
    let mut fb = System::allocate_framebuffer(&config);
    let mut asnd = Asnd::init();
    let mut player = MP3Player::new(asnd);

    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);

    video::configure(&config);
    video::set_next_framebuffer(&fb);
    video::set_black(false);
    video::flush();
    video::wait_vsync();

    Console::init(fb, &config);
    println!("Hello World!");

    player.play_buffer(mp3);

    loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);

        if gcn_ctrl.is_button_down(Button::Start) {
            break 0;
        }

        if wii_ctrl.is_button_down(Button::Home) {
            break 0;
        }

        video::wait_vsync();
    }
}
