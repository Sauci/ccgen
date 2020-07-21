#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Edge {
    Falling,
    Rising,
}

impl core::ops::Not for Edge {
    type Output = Edge;

    fn not(self) -> Self::Output {
        match self {
            Edge::Rising => Edge::Falling,
            Edge::Falling => Edge::Rising,
        }
    }
}

pub const REV_DEG_TICKS: u32 = 3_600;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub id: u8,
    pub ag: u32,
    pub edge: Edge,
    pub is_gen: bool,
}

impl Event {
    pub const fn new() -> Event {
        Event {
            id: 0,
            ag: 0,
            edge: Edge::Falling,
            is_gen: true,
        }
    }
}
