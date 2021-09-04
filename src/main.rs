#[macro_use]
extern crate glium;
extern crate glium_glyph;
extern crate glyph_brush;

mod game;
mod player;
mod state;
mod shop;
mod combat;
mod matrix_transforms;

use glium::glutin::window::{Window, WindowBuilder, WindowAttributes};
use glium::glutin::{Api, GlProfile, GlRequest};
use glium::{glutin, Surface};

use glium_glyph::glyph_brush::{rusttype::Font, Section, Layout, HorizontalAlign, rusttype::Scale};
use glium_glyph::GlyphBrush;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::io::Cursor;
use player::{Player};

fn main() {

    let event_loop = glutin::event_loop::EventLoop::new();
    let window = WindowBuilder::new().with_title("Waridor".to_owned());
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../graphics/main.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let font_name: &[u8] = include_bytes!("../resources/Freedom.ttf");
    let fonts = vec![Font::from_bytes(font_name).unwrap()];
    let mut glyph_brush = GlyphBrush::new(&display, fonts);

    let mut game = game::Game::default();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-0.0625, -0.0625], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ -0.0625,  0.0625], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.0625, 0.0625], tex_coords: [1.0, 1.0] };
    let vertex4 = Vertex { position: [ 0.0625, -0.0625], tex_coords: [1.0, 0.0] };
    
    let shape = vec![vertex1, vertex2, vertex4, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform MatrixBlock
        {
          mat4 m[16];
        } matrices;
        
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
            gl_Position = gl_Position;
            for(int i=0;i < 16;++i)
            {
                int j = i;
                gl_Position = matrices.m[j] * gl_Position;
            }
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput{ input, .. } => {
                    game.input(input);
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        game.update();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let identity_matrix = matrix_transforms::IDENTITY_MATRIX;
        let rotate_y_matrix = matrix_transforms::rotate_around_y_axis(game.player.pos);
        let move_y_matrix = matrix_transforms::translate([0.0, -0.5, 0.0]);
        let rotate_z_matrix = matrix_transforms::rotate_around_z_axis(game.player.pos);

        let matrices: glium::uniforms::UniformBuffer<[[[f32; 4]; 4]; 16]>;
        let result = glium::uniforms::UniformBuffer::new(&display, [move_y_matrix, rotate_y_matrix,rotate_z_matrix,identity_matrix,identity_matrix, identity_matrix,identity_matrix,identity_matrix,identity_matrix, identity_matrix,identity_matrix,identity_matrix,identity_matrix, identity_matrix,identity_matrix,identity_matrix]);
        match result{
            Ok(v) => matrices = v,
            Err(_e) =>{
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            }
        }

        let uniforms = uniform! {
            MatrixBlock : &matrices,
            tex: &texture,
        };

        game.state.draw(&mut game.player, &mut glyph_brush, &display);
        glyph_brush.draw_queued(&display, &mut target);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}