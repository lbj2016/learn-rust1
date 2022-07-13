pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait Duration {
    fn duration(&self) -> u8;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match &self {
            Self::Red => 45,
            Self::Green => 60,
            Self::Yellow => 10,
        }
    }
}

pub fn get_duration<T: Duration>(light: &T) -> u8 {
    light.duration()
}
