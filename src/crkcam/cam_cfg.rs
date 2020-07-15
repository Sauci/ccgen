use super::cam::CamCfg;
use super::cmn::*;

pub const CAM_CONFIGS: [CamCfg; 1] = [CamCfg {
    ev_nr: 21,
    ev_ag: [
        (2890,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (8000,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (2000,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (5000,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (5000,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (11000, Edge::Falling), 
        (1000,  Edge::Rising), 
        (11000, Edge::Falling), 
        (1000,  Edge::Rising), 
        (5000,  Edge::Falling),
        (1000,  Edge::Rising), 
        (5000,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (2000,  Edge::Falling), 
        (1000,  Edge::Rising), 
        (5110,  Edge::Rising),
    ],
}];
