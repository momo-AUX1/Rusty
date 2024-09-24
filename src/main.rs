extern crate sdl2;
extern crate gl;

use glow::HasContext;
use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
//use sdl2::pixels::Color;
use sdl2::sys::exit;
use std::fs;
use std::time::Duration;

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let sdl2_video_context = sdl2_context.video().unwrap();

    let window = sdl2_video_context.window("SDL2+OpenGL Rust", 800, 500)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    //let mut canvas = window.unwrap().into_canvas().build().unwrap_or_else(|e| panic!("{}", e));
    //canvas.set_draw_color(Color::RGB(0, 255, 0));
    //canvas.clear();
    //canvas.present();

    let gl_attr = sdl2_video_context.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    let _gl_context = window.gl_create_context().unwrap();
    let gl = unsafe { glow::Context::from_loader_function(|s| sdl2_video_context.gl_get_proc_address(s) as *const _) };

    unsafe {
        let vertices : [f32; 9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ];

        let vertex_shader_source = fs::read_to_string("src/vertex.glsl").unwrap();
        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, &vertex_shader_source);
        gl.compile_shader(vertex_shader);

        let fragment_shader_source = fs::read_to_string("src/fragment.glsl").unwrap();
        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, &fragment_shader_source);
        gl.compile_shader(fragment_shader);
        

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        gl.use_program(Some(program));

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertices.align_to().1, glow::STATIC_DRAW);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            0,                     
            3,                     
            glow::FLOAT,            
            false,                 
            3 * std::mem::size_of::<f32>() as i32, 
            0                       
        );

        gl.viewport(0, 0, 800, 500);
        gl.clear_color(0.1, 0.1, 0.1, 1.0);


        let mut event_pump = sdl2_context.event_pump().unwrap();

        loop {
            for event in event_pump.poll_iter(){
                println!("{:?}", event);

                match event {
                    Event::Quit { timestamp } => {
                        println!("{:?}", timestamp);
                        println!("QUIT");
                         exit(0);
                    }

                    _ => {}
                }

                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.draw_arrays(glow::TRIANGLES, 0, 3);
                window.gl_swap_window();

                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        }
    }
}