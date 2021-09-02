pub struct Shop{
}

impl crate::state::State for Shop{
    fn input(&mut self, player : &mut crate::player::Player, input : crate::glutin::event::KeyboardInput){
    let virtual_keycode = input.virtual_keycode.unwrap();
    match virtual_keycode{
        //Game related input
        crate::glutin::event::VirtualKeyCode::Up =>{

        },
        crate::glutin::event::VirtualKeyCode::Down =>{

        },
        crate::glutin::event::VirtualKeyCode::Return =>{
            
        },
        _=>return,
    }
    }
    fn update(&mut self, player : &mut crate::player::Player){}
    fn draw(&mut self, player : &mut crate::player::Player, glyph_brush : &mut crate::GlyphBrush, display : &crate::glium::Display){
        let screen_dims = display.get_framebuffer_dimensions();

        glyph_brush.queue(crate::Section {
            text: "\n\nPlasma\nLaser\nSpeed Boost\nFire rate\nExtra life",
            scale: crate::Scale::uniform(screen_dims.0 as f32 * 0.04),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.1),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: crate::Layout::default().h_align(crate::HorizontalAlign::Center),
            ..crate::Section::default()
        });
        glyph_brush.queue(crate::Section {
            text: "Shop",
            scale: crate::Scale::uniform(screen_dims.0 as f32 * 0.06),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.1),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: crate::Layout::default().h_align(crate::HorizontalAlign::Center),
            ..crate::Section::default()
        });
    }
}