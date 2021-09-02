extern crate glium;
extern crate glium_glyph;
extern crate glyph_brush;

use glium::glutin::{Api, GlProfile, GlRequest};
use glium::{glutin, Surface};

use glium_glyph::glyph_brush::{rusttype::Font, Section, Layout, HorizontalAlign, rusttype::Scale};
use glium_glyph::GlyphBrush;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};

pub fn main() {
    let event_loop = EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
        .with_srgb(true);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let font_name: &[u8] = include_bytes!("../graphics/Freedom.ttf");
    let fonts = vec![Font::from_bytes(font_name).unwrap()];
    let mut glyph_brush = GlyphBrush::new(&display, fonts);

    event_loop.run(move |event, _tgt, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
        let screen_dims = display.get_framebuffer_dimensions();

        glyph_brush.queue(Section {
            text: "\n\nPlasma\nLaser\nSpeed Boost\nFire rate\nExtra life",
            scale: Scale::uniform(screen_dims.0 as f32 * 0.04),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.1),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: Layout::default().h_align(HorizontalAlign::Center),
            ..Section::default()
        });
        glyph_brush.queue(Section {
            text: "Shop",
            scale: Scale::uniform(screen_dims.0 as f32 * 0.06),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.1),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: Layout::default().h_align(HorizontalAlign::Center),
            ..Section::default()
        });

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        glyph_brush.draw_queued(&display, &mut target);
        target.finish().unwrap();
    });
}