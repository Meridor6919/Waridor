#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use std::io::Cursor;

enum MoveDirections{
    Left,
    Right,
    None
}
struct Player{
    moving : MoveDirections,
    pos : f32,
}
impl Player{
    fn input(&mut self, input : glutin::event::KeyboardInput) -> bool{
        let f = input.virtual_keycode.unwrap();
        match f{
            //Game related input
            glutin::event::VirtualKeyCode::Left =>{
                if input.state == glutin::event::ElementState::Pressed {self.moving = MoveDirections::Left} else {self.moving = MoveDirections::None} 
                return false;
            },
            glutin::event::VirtualKeyCode::Right =>{
                if input.state == glutin::event::ElementState::Pressed {self.moving = MoveDirections::Right} else {self.moving = MoveDirections::None} 
                return false;
            },
            glutin::event::VirtualKeyCode::LControl =>{
                return false;
            },
            _=>return true,
        }
    }
    fn update(&mut self) -> (){
        match self.moving{
            MoveDirections::Left =>{
                self.pos -= 0.02
            },
            MoveDirections::Right =>{
                self.pos += 0.02
            },
            _=> return,
        }
    }
}

struct Game{
}
impl Game{
    fn input(&self, input : glutin::event::KeyboardInput) -> bool{
        let f = input.virtual_keycode.unwrap();
        match f{
            //Game related input
            glutin::event::VirtualKeyCode::Return =>{
                return false;
            },
            glutin::event::VirtualKeyCode::R =>{
                return false;
            },
            _=>return true,
        }
    }
}
struct Shop{
}
impl Shop{
    fn input(&self, input : glutin::event::KeyboardInput) -> bool{
        let f = input.virtual_keycode.unwrap();
        match f{
            //Game related input
            glutin::event::VirtualKeyCode::Up =>{
                return false;
            },
            glutin::event::VirtualKeyCode::Down =>{
                return false;
            },
            glutin::event::VirtualKeyCode::Return =>{
                return false;
            },
            _=>return true,
        }
    }
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../graphics/main.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

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

    let mut player = Player{moving : MoveDirections::None, pos : 0.5};
    let mut shop =Shop{};
    let mut game = Game{};

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
                    if game.input(input)
                    {
                        if shop.input(input)
                        {
                            if player.input(input)
                            {
                                return;
                            }
                        }
                    }
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
        player.update();
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ player.pos , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}