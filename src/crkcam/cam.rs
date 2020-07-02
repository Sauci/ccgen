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
    pub mai_edge: Edge,
    pub ev_ag: [u32; 21],
}

pub struct CamWheel {
    pub ev: Vec<AgEv, U21>,
    pub cfg: &'static CamCfg,
}

impl CamWheel {
    pub fn new(cfg: &'static CamCfg) -> CamWheel {
        let mut cam = CamWheel {
            ev: Vec::new(),
            cfg,
        };

        cam.cfg.ev_ag.iter().enumerate().for_each(|(idx, ag)| {
            let ev = AgEv {
                ag: *ag,
                edge: {
                    if idx % 2 == 0 {
                        !cam.cfg.mai_edge
                    } else {
                        cam.cfg.mai_edge
                    }
                },
                is_gen: true,
            };
            cam.ev
                .push(ev)
                .expect("Too much events for camshaft configuration.");
        });
        cam
    }
}

pub struct CamSigGen {
    gen_pos: usize,
    norm_dir: bool,
    wheel: Option<CamWheel>,
}

impl CamSigGen {
    pub const fn new() -> CamSigGen {
        CamSigGen {
            gen_pos: 0,
            norm_dir: true,
            wheel: None,
        }
    }

    pub fn set_cam(&mut self, cam: &'static CamCfg) {
        self.wheel = Some(CamWheel::new(cam));
    }
}

impl Iterator for CamSigGen {
    type Item = AgEv;

    fn next(&mut self) -> Option<Self::Item> {
        if self.wheel.is_some() {
            let cam = self.wheel.as_ref().unwrap();
            let ev = cam.ev[self.gen_pos];
            if self.norm_dir {
                self.gen_pos += 1;
                if self.gen_pos >= cam.cfg.ev_nr {
                    self.gen_pos = 0;
                }
            } else {
                if self.gen_pos == 0 {
                    self.gen_pos = cam.cfg.ev_nr - 1;
                } else {
                    self.gen_pos -= 1;
                }
            }
            Some(ev)
        } else {
            None
        }
    }
}
