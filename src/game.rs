pub struct Game{
    pub state_type : crate::state::StateTypes,
    pub state : Box<dyn crate::state::State>,
    pub player : crate::player::Player
}
impl Game{
    pub fn input(&mut self, input : crate::glutin::event::KeyboardInput){
        if input.virtual_keycode == None{
            return;
        }
        let f = input.virtual_keycode.unwrap();
        match f{
            //Game related input
            crate::glutin::event::VirtualKeyCode::S =>{
                self.state = Box::new(crate::shop::Shop::default());
                self.state_type = crate::state::StateTypes::shop;
            },
            crate::glutin::event::VirtualKeyCode::C =>{
                self.state = Box::new(crate::combat::Combat::default());
                self.state_type = crate::state::StateTypes::combat;
            },
            _=>self.state.input(&mut self.player, input),
        }
    }
    pub fn update(&mut self){
        if self.state.update(&mut self.player){
            if matches!(self.state_type, crate::state::StateTypes::shop){
                self.state = Box::new(crate::combat::Combat::default());
                self.state_type = crate::state::StateTypes::combat;
            }
            else{
                self.state = Box::new(crate::shop::Shop::default());
                self.state_type = crate::state::StateTypes::shop;
            }
        }
    }
}
impl Default for Game {
    fn default() -> Game {
        Game {
            state : Box::new(crate::combat::Combat::default()),
            state_type : crate::state::StateTypes::combat,
            player : crate::player::Player::default()
        }
    }
}