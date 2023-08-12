use super::KeyboardInput;

#[derive(Debug)]
pub struct Event<E, T> {
    pub event: E, //NS pub
    pub time: T,  //same
}

impl<E, T> Event<E, T> {
    pub fn new(event: E, time: T) -> Event<E, T> {
        Self { event, time }
    }
}

impl<E, T> From<(E, T)> for Event<E, T> {
    fn from(value: (E, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<(u8, u8, T)> for Event<KeyboardInput, T> {
    fn from(value: (u8, u8, T)) -> Self {
        Self::new((value.0, value.1).into(), value.2)
    }
}

impl<T> Event<KeyboardInput, T> {
    pub fn down(code: u8, time: T) -> Self {
        Self::new(KeyboardInput { code, state: 1 }, time)
    }

    pub fn up(code: u8, time: T) -> Self {
        Self::new(KeyboardInput { code, state: 2 }, time)
    }

    pub fn none(time: T) -> Self {
            Self::new(KeyboardInput::default(), time)
    }
}
