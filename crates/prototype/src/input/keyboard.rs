#[derive(Debug, Default)]
pub struct KeyboardInput {
    /// key code
    pub code: u8,
    /// pressed, released or something else
    pub state: u8,
}

// note: default KeyboardInput indicates that there is no actual input

impl From<(u8, u8)> for KeyboardInput {
    fn from(value: (u8, u8)) -> Self {
        KeyboardInput {
            code: value.0,
            state: value.1,
        }
    }
}
