#[macro_use]
extern crate glium;
extern crate glium_glyph;
extern crate glyph_brush;

mod player;
mod state;
mod shop;
mod combat;

use glium::glutin::window::{Window, WindowBuilder, WindowAttributes};
use glium::glutin::{Api, GlProfile, GlRequest};
use glium::{glutin, Surface};

use glium_glyph::glyph_brush::{rusttype::Font, Section, Layout, HorizontalAlign, rusttype::Scale};
use glium_glyph::GlyphBrush;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::io::Cursor;
use player::{Player};

struct Game{
    state_type : state::StateTypes,
    state : Box<dyn state::State>,
    player : player::Player
}
impl Game{
    fn input(&mut self, input : glutin::event::KeyboardInput){
        if input.virtual_keycode == None{
            return;
        }
        let f = input.virtual_keycode.unwrap();
        match f{
            //Game related input
            glutin::event::VirtualKeyCode::S =>{
                self.state = Box::new(shop::Shop::default());
                self.state_type = state::StateTypes::shop;
            },
            glutin::event::VirtualKeyCode::C =>{
                self.state = Box::new(combat::Combat::default());
                self.state_type = state::StateTypes::combat;
            },
            _=>self.state.input(&mut self.player, input),
        }
    }
    fn update(&mut self){
        if self.state.update(&mut self.player){
            if matches!(self.state_type, state::StateTypes::shop){
                self.state = Box::new(combat::Combat::default());
                self.state_type = state::StateTypes::combat;
            }
            else{
                self.state = Box::new(shop::Shop::default());
                self.state_type = state::StateTypes::shop;
            }
        }
    }
}
impl Default for Game {
    fn default() -> Game {
        Game {
            state : Box::new(combat::Combat::default()),
            state_type : state::StateTypes::combat,
            player : player::Player::default()
        }
    }
}

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

    let mut game = Game::default();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex { position: [-0.0625, -1.0], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ -0.0625,  -0.875], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.0625, -0.875], tex_coords: [1.0, 1.0] };
    let vertex4 = Vertex { position: [ 0.0625, -1.0], tex_coords: [1.0, 0.0] };
    
    let shape = vec![vertex1, vertex2, vertex4, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
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

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ game.player.pos , 0.0, 0.0, 1.0f32],
            ],
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