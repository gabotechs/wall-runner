pub struct LevelStatus {
    pub current_level: String,
    pub current_win_z: f32,
}

impl Default for LevelStatus {
    fn default() -> Self {
        LevelStatus {
            current_win_z: 999999.9,
            current_level: String::from(""),
        }
    }
}
