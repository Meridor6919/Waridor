
pub enum StateTypes{
    shop,
    combat,
}
pub trait State {
    fn input(&mut self, player : &mut crate::player::Player, input : crate::glutin::event::KeyboardInput);
    fn update(&mut self, player : &mut crate::player::Player) -> bool;
    fn draw(&mut self, player : &mut crate::player::Player, glyph_brush : &mut crate::GlyphBrush, display : &crate::glium::Display);
}