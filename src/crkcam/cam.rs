use super::cmn::*;
use core::iter::Iterator;
use heapless::consts::U21;
use heapless::Vec;

/// CamCfg, shall be configured in the following manner:
/// Level
/// ^
/// 0r     1f     2r               3f             4r
/// |------+      +----------------+              +------
/// |  ag0 |  ag1 |      ag2       |     ag3      |  ag4
/// |      |      |                |              |
/// +------+------+----------------+--------------+------> Ag
/// The first angle shall be calculated from the reference, aka crank gape.
/// The last angle shall be calculated from the last edge to 720°.
/// "r" and "f" on event ids stand for "rising" or "falling".
/// The configuration shown above isn't real, only example purpose.
pub struct CamCfg {
    pub ev_nr: usize,
    pub ev_ary: [(u32, Edge); 21],
}

pub struct CamWheel {
    pub ev: Vec<Event, U21>,
}

impl CamWheel {

    /// Create a camshaft wheel from a configuration of angles, edges and number of events
    /// 
    /// This function cannot fail as the configuration uses exactly the same number 
    /// of events as the wheel. No check is performed on the configuration. It might be an 
    /// impossible wheel configuration that is passed to it. 
    pub fn new(cfg: &CamCfg) -> CamWheel {
        let mut cam = CamWheel {
            ev: Vec::new(),
        };

        for (idx, ag) in cfg.ev_ary.iter().enumerate() {
            let ev = Event {
                id: idx as u8,
                ag: ag.0,
                edge: ag.1,
                is_gen: true,
            };
            match cam.ev.push(ev) {
                Ok(()) => (),
                Err(_) => (), 
            };
        }

        cam
    }
}

pub struct CamSigGen {
    gen_pos: usize,
    cam: CamWheel,
}

impl CamSigGen {
    pub fn new(cam: &CamCfg) -> Result<CamSigGen, ()> {
        Ok(
            CamSigGen {
                gen_pos: 0,
                cam: CamWheel::new(cam),
            }
        )
    }
}

impl Iterator for CamSigGen {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let ev = self.cam.ev[self.gen_pos];
        self.gen_pos += 1;
        if self.gen_pos >= self.cam.ev.len() {
            self.gen_pos = 0;
        }
        Some(ev)
    }
}
