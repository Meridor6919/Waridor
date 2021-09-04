pub struct Player{
    pub cash : u16,
    pub movement_speed : f32,
    pub fire_rate : f32,
    pub pos : f32,
}
impl Default for Player {
    fn default() -> Player {
        Player {
            cash : 0,
            movement_speed : 0.025,
            fire_rate : 1.0,
            pos : 0.0,
        }
    }
}