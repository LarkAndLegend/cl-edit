extern crate native;
extern crate glfw;

use glfw::Context;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

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
    let (window, events) = glfw.create_window(300,300,"Cosmolark Asset Editor",glfw::Windowed)
                               .expect("Failed to open window.");

    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_cursor_enter_polling(true);
    window.set_size_polling(true);
    window.set_char_polling(true);
    window.set_mouse_button_polling(true);

    window.make_current();
    window.set_cursor_mode(glfw::CursorNormal);

    while !window.should_close() {
        glfw.poll_events();
        for (_,event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }
} // end function main()


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











