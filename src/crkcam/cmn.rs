pub fn spd_ag_to_ti(spd: u32, ag: u32) -> u16 {
    ((ag as u64 * 1_000_000) / (spd as u64 * 6)) as u16
}

pub fn spd_ti_to_ag(spd: u32, ti: u32) -> u32 {
    ((spd as u64 * 36_000) / (ti as u64 * 60_000_000)) as u32
}

pub fn ti_ag_to_spd(ag: u32, ti: u32) -> u32 {
    ((ag as u64 * 60_000_000) / (ti as u64 * 36_000)) as u32
}

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

pub const REV_ANGLE: u32 = 36_000;

#[derive(Debug, Copy, Clone)]
pub struct AgEv {
    pub ag: u32,
    pub edge: Edge,
    pub is_gen: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct TiEv {
    pub ti: u32,
    pub edge: Edge,
    pub is_gen: bool,
}
