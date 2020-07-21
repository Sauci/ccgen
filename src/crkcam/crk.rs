use super::cmn::*;

use heapless::consts::U240;
use heapless::Vec;

use core::iter::Iterator;

#[derive(Debug)]
pub struct CrkCfg {
    pub tooth_nr: u8,
    pub miss_tooth_nr: u8,
    pub mai_edge: Edge,
}

impl CrkCfg {
    pub const fn new(tooth_nr: u8, miss_tooth_nr: u8, mai_edge: Edge) -> CrkCfg {
        CrkCfg {
            tooth_nr,
            miss_tooth_nr,
            mai_edge,
        }
    }
}

#[derive(Debug)]
pub struct CrkWheel {
    pub ev: Vec<Event, U240>,
    pub cfg: &'static CrkCfg,
}

impl CrkWheel {
    pub fn new(cfg: &'static CrkCfg) -> CrkWheel {
        let mut crk = CrkWheel {
            ev: Vec::new(),
            cfg,
        };

        let tmp_tooth_ag = REV_DEG_TICKS / crk.cfg.tooth_nr as u32;
        for idx in 0..(crk.cfg.tooth_nr * 2) {
            crk.ev.push({
                let angle = tmp_tooth_ag / 2;
                let is_gen = if idx < (crk.cfg.miss_tooth_nr * 2) + 1 && idx > 0 {
                    false
                } else {
                    true
                };
                let edge = if idx % 2 == 0 {
                    crk.cfg.mai_edge
                } else {
                    !crk.cfg.mai_edge
                };
                Event {
                    id: idx,
                    ag: angle,
                    edge,
                    is_gen,
                }
            }).unwrap();
        }

        crk
    }

    pub fn teeth_nr(&self) -> u8 {
        self.cfg.tooth_nr
    }
}

pub struct CrkSigGen {
    gen_pos: usize,
    crk: CrkWheel,
}

impl CrkSigGen {
    pub fn new(cfg: &'static CrkCfg) -> CrkSigGen {
        CrkSigGen {
            gen_pos: 0,
            crk: CrkWheel::new(cfg),
        }
    }
}

impl Iterator for CrkSigGen {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let ev = self.crk.ev[self.gen_pos];
        self.gen_pos += 1;
        if self.gen_pos >= (self.crk.teeth_nr() * 2) as usize {
            self.gen_pos = 0;
        }
        Some(ev)
    }
}
