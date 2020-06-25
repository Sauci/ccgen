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
    pub ev: Vec<AgEv, U240>,
    pub cfg: &'static CrkCfg,
}

impl CrkWheel {
    pub fn new(cfg: &'static CrkCfg) -> CrkWheel {
        let mut crk = CrkWheel {
            ev: Vec::new(),
            cfg,
        };

        let tmp_tooth_ag = REV_ANGLE / crk.cfg.tooth_nr as u32;
        for idx in 0..(crk.cfg.tooth_nr * 2) {
            crk.ev
                .push({
                    let angle = tmp_tooth_ag / 2;
                    let is_gen = if idx < (crk.cfg.tooth_nr - crk.cfg.miss_tooth_nr) * 2 {
                        true
                    } else {
                        false
                    };
                    let edge = if idx % 2 == 0 {
                        crk.cfg.mai_edge
                    } else {
                        !crk.cfg.mai_edge
                    };
                    AgEv {
                        ag: angle,
                        edge,
                        is_gen,
                    }
                })
                .unwrap();
        }

        crk
    }

    pub fn teeth_nr(&self) -> u8 {
        self.cfg.tooth_nr
    }
}

pub struct CrkSigGen {
    gen_pos: usize,
    crk: Option<CrkWheel>,
}

impl CrkSigGen {
    pub const fn new() -> CrkSigGen {
        CrkSigGen {
            gen_pos: 0,
            crk: None,
        }
    }

    pub fn set_crk(&mut self, cfg: &'static CrkCfg) {
        self.gen_pos = 0;
        self.crk = Some(CrkWheel::new(cfg));
    }
}

impl Iterator for CrkSigGen {
    type Item = AgEv;

    fn next(&mut self) -> Option<Self::Item> {
        if self.crk.is_some() {
            let crk = self.crk.as_ref().unwrap();
            let ev = crk.ev[self.gen_pos];
            self.gen_pos += 1;
            if self.gen_pos >= crk.teeth_nr() as usize {
                self.gen_pos = 0;
            }
            Some(ev)
        } else {
            None
        }
    }
}
