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
extern crate libc;
extern crate log;
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
    
            sdl2::video::gl_set_attribute(sdl2::video::GLContextMajorVersion,3);
        sdl2::video::gl_set_attribute(sdl2::video::GLContextMinorVersion,0);
        sdl2::video::gl_set_attribute(sdl2::video::GLDepthSize,24);
        sdl2::video::gl_set_attribute(sdl2::video::GLDoubleBuffer, 1);
        sdl2::video::gl_set_attribute(sdl2::video::GLContextProfileMask,sdl2::video::ll::SDL_GL_CONTEXT_PROFILE_CORE as int);

    // open a window
    let window = match sdl2::video::Window::new("Cosmolark Asset Editor",sdl2::video::PosCentered, sdl2::video::PosCentered, 300,300,sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let context = window.gl_create_context().unwrap();
    sdl2::clear_error();
    gl::load_with(|name| {
        match sdl2::video::gl_get_proc_address(name) {
            Some(glproc) => glproc as *const libc::c_void,
            None         => { println!("missing GL function: {}", name); std::ptr::null() }
        }
    });

    window.show();
    


    unsafe {
        gl::ClearColor(0.3,0.3,0.3,1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    
    // swap buffer
    window.gl_swap_window();
    

    'event : loop {
        match sdl2::event::poll_event() {
            sdl2::event::Quit(_) => break 'event,
            sdl2::event::None    => continue,
            event                => println!("event: {}", event),
        }
    }
    
    sdl2::quit();
} // end function main()















