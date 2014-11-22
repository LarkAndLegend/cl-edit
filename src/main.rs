// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// Copyright 2014 Mark McDermott.
//
// Licensed under the the MIT license
// <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

extern crate sdl2;
extern crate gl;
extern crate native;


//use sdl2::video::{Window, PosCentered, OPENGL};
//use sdl2::event::{Quit, None, poll_event};

// --------------------------------------------------------------------------
// Native entry point
// --------------------------------------------------------------------------
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}



// --------------------------------------------------------------------------
// The "Real" entry point
// --------------------------------------------------------------------------
fn main() {
    println!("Welcome to the the Cosmolark editor!")
    
    sdl2::init(sdl2::INIT_EVERYTHING);
    
    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });


    // open a window
    let window = match sdl2::video::Window::new("Cosmolark Asset Editor",sdl2::video::PosCentered, sdl2::video::PosCentered, 300,300,sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    window.show();
    
    // create a rendering context
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };
    
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(101,208,246)); // light blue
    let _ = renderer.clear();
    
    let _ = renderer.set_draw_color(sdl2::pixels::RGB(0,153,204)); // dark blue
    let border_rect = sdl2::rect::Rect::new(320-64, 240-64, 128, 128);
    let _ = match renderer.draw_rect(&border_rect) {
        Ok(_) => {},
        Err(err) => panic!("failed to draw rect: {}", err)
    };
    
    // swap buffer
    let _ = renderer.present();
    

    'event : loop {
        match sdl2::event::poll_event() {
            sdl2::event::Quit(_) => break 'event,
            sdl2::event::None    => continue,
            event                => println!("event: {}", event),
        }
    }
    
    sdl2::quit();
} // end function main()















