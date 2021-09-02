pub struct Combat{
    closing_request : bool,

    pub moving_left : bool,
    pub moving_right : bool,
    pub shooting: bool,
}
impl Default for Combat {
    fn default() -> Combat {
        Combat {
            closing_request : false,
            moving_left : false,
            moving_right : false,
            shooting: false,
        }
    }
}

impl crate::state::State for Combat{
    fn input(&mut self, player : &mut crate::player::Player, input : crate::glutin::event::KeyboardInput){
        let f = input.virtual_keycode.unwrap();
        match f{
            //Game related input
            crate::glutin::event::VirtualKeyCode::Left =>{
                self.moving_left = input.state == crate::glutin::event::ElementState::Pressed;
            },
            crate::glutin::event::VirtualKeyCode::Right =>{
                self.moving_right = input.state == crate::glutin::event::ElementState::Pressed;
            },
            crate::glutin::event::VirtualKeyCode::LControl =>{
                self.shooting = input.state == crate::glutin::event::ElementState::Pressed;
            },
            _=>return,
        }
    }
    fn update(&mut self, player : &mut crate::player::Player) -> bool{
        if self.moving_left{
            player.pos -= player.movement_speed;
        }
        if self.moving_right{
            player.pos += player.movement_speed;
        }
        return self.closing_request;
    }
    fn draw(&mut self, player : &mut crate::player::Player, glyph_brush : &mut crate::GlyphBrush, display : &crate::glium::Display){
    }
}