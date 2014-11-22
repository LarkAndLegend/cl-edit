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

use gl::types::{GLfloat,GLuint,GLint,GLchar,GLenum,GLboolean,GLsizeiptr};
use std::ptr;
use std::str;
use std::mem;

// Vertex data
static VERTEX_DATA: [GLfloat, ..6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";


fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}


fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
        }
    }
    program
}}




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

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);
    
    sdl2::video::gl_set_swap_interval(0);
    window.show();
    
    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                      (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      mem::transmute(&VERTEX_DATA[0]),
                      gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                               gl::FALSE as GLboolean, 0, ptr::null());
    }



    let mut cnt: u64 = 0;
    'event : loop {
                
        unsafe {
            gl::ClearColor(0.3,0.3,0.3,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0,3);
        }
        
        window.gl_swap_window();
        
        cnt = cnt + 1;
        
        if cnt % 100 == 0 {
            println!("Cnt: {}",cnt);
        }
        
        match sdl2::event::poll_event() {
            sdl2::event::Quit(_) => break 'event,
            sdl2::event::None    => continue,
            //event                => println!("event: {}", event),
            _                    => continue
        }
    }
    
    unsafe {
        // Cleanup
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
    sdl2::quit();
} // end function main()















