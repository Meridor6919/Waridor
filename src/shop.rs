const SHOP_ITEMS: &'static [&'static str] = &["Back", "Movement boost", "Fire boost", "Double shot", "Triple shot", "Laser", "Extra life"];

pub struct Shop{
    index : u8,
    closing_request : bool
}
impl Default for Shop {
    fn default() -> Shop {
        Shop {
            index : 0,
            closing_request : false
        }
    }
}

impl crate::state::State for Shop{
    fn input(&mut self, player : &mut crate::player::Player, input : crate::glutin::event::KeyboardInput){
    let virtual_keycode = input.virtual_keycode.unwrap();
    match virtual_keycode{
        //Game related input
        crate::glutin::event::VirtualKeyCode::Up =>{
            if self.index + 1 < SHOP_ITEMS.len() as u8 && input.state == crate::glutin::event::ElementState::Released{
                self.index += 1;
            }
        },
        crate::glutin::event::VirtualKeyCode::Down =>{
            if self.index > 0 && input.state == crate::glutin::event::ElementState::Released{
                self.index -= 1;
            }
        },
        crate::glutin::event::VirtualKeyCode::Return =>{
            if self.index == 0{
                self.closing_request = true;
            }
        },
        _=>return,
    }
    }
    fn update(&mut self, player : &mut crate::player::Player) -> bool{
        return self.closing_request;
    }
    fn draw(&mut self, player : &mut crate::player::Player, glyph_brush : &mut crate::GlyphBrush, display : &crate::glium::Display){
        let screen_dims = display.get_framebuffer_dimensions();

        let mut menu_text  = "\n\n\n\n".to_owned();
        let mut highlighted_text  = "\n\n\n\n".to_owned();

        let mut i = SHOP_ITEMS.len() - 1;
        while true{
            menu_text = format!("{}\n{}", menu_text, SHOP_ITEMS[i]);
            if i == self.index as usize{
                highlighted_text = format!("{}\n{}", highlighted_text, SHOP_ITEMS[i]);
            }
            else{
                highlighted_text = format!("{}\n", highlighted_text);
            }
            if i == 0{
                break;
            }
            i = i - 1;
        }
        

        glyph_brush.queue(crate::Section {
            text: &menu_text,
            scale: crate::Scale::uniform(screen_dims.0 as f32 * 0.04),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.05),
            color: [0.5, 0.5, 0.5, 1.0],
            layout: crate::Layout::default().h_align(crate::HorizontalAlign::Center),
            ..crate::Section::default()
        });
        glyph_brush.queue(crate::Section {
            text: &highlighted_text,
            scale: crate::Scale::uniform(screen_dims.0 as f32 * 0.04),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.05),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: crate::Layout::default().h_align(crate::HorizontalAlign::Center),
            ..crate::Section::default()
        });
        glyph_brush.queue(crate::Section {
            text: "Shop",
            scale: crate::Scale::uniform(screen_dims.0 as f32 * 0.1),
            screen_position: (screen_dims.0 as f32 * 0.5, screen_dims.1 as f32 * 0.05),
            color: [1.0, 1.0, 1.0, 1.0],
            layout: crate::Layout::default().h_align(crate::HorizontalAlign::Center),
            ..crate::Section::default()
        });
    }
}