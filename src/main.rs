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


extern crate native;
extern crate glfw;
extern crate gl;

use glfw::Context;

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
    
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // print out available modes
    glfw.with_primary_monitor(|monitor| {
        let _ = monitor.map(|monitor| {
            println!("{}:", monitor.get_name());
            println!("    {}\n",monitor.get_video_mode().unwrap());
        });
    });

    println!("Available Monitors\n------------------");
    glfw.with_connected_monitors(|monitors| {
        for monitor in monitors.iter() {
            println!("{}:", monitor.get_name());
            for mode in monitor.get_video_modes().iter() {
                println!("    {}",*mode);
            }
        }
    });


    // open a window
    glfw.window_hint(glfw::ContextVersion(3,2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let (window, events) = glfw.create_window(300,300,"Cosmolark Asset Editor",glfw::Windowed)
                               .expect("Failed to open window.");

    // set up GL 
    gl::load_with(|s| window.get_proc_address(s));
    gl::ClearColor(0.3,0.3,0.3,1.0);
    glfw.set_swap_interval(0); // Turn off vertical sync

    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_cursor_enter_polling(true);
    window.set_size_polling(true);
    window.set_char_polling(true);
    window.set_mouse_button_polling(true);

    window.make_current();
    window.set_cursor_mode(glfw::CursorNormal);

    let mut frame_cnt: u64 = 0;
    let mut prev_time: f64 = glfw.get_time();

    while !window.should_close() {
        glfw.poll_events();
        for (_,event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }

        // count frames
        frame_cnt += 1;
        if 0 == frame_cnt % 100 {
            let curr_time = glfw.get_time();
            println!("Frames Per Second = {}", 100.0/(curr_time-prev_time));
            prev_time = curr_time;
        }

        
        gl::Clear(gl::COLOR_BUFFER_BIT);
        window.swap_buffers();
    }
} // end function main()



// --------------------------------------------------------------------------
// Monolithic event handler function. TODO: refactor into proper event 
// handling mechanism.
// --------------------------------------------------------------------------
fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        },
        glfw::CursorPosEvent(x,y) => {
            println!("Cursor position ({},{})",x,y)
        },
        glfw::CursorEnterEvent(entered) => {
            match entered {
                true => println!("Cursor entered!"),
                _    => println!("Cursor exited."),
            }
        }
        glfw::SizeEvent(x,y) => {
            println!("New size {}x{}",x,y)
        },
        glfw::MouseButtonEvent(button,action,_) => {
           
            let button_ix = match button {
                glfw::MouseButton1 => 1u,
                glfw::MouseButton2 => 2u,
                glfw::MouseButton3 => 3u,
                _                  => 0u,
            };

            if 0 < button_ix {
                println!("Mouse event button {} {}",button_ix,action)
            }
        },
        glfw::CharEvent(c) => {
            println!("{}",c)
        },
        _ => {},
    }
} // end funtion handle_window_event()











