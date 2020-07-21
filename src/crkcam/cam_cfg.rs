use super::cam::CamCfg;
use super::cmn::*;

pub const CAM_CONFIGS: [CamCfg; 1] = [CamCfg {
    ev_nr: 21,
    ev_ag: [
        (289,  Edge::Falling), 
        (100,  Edge::Rising), 
        (800,  Edge::Falling), 
        (100,  Edge::Rising), 
        (200,  Edge::Falling), 
        (100,  Edge::Rising), 
        (500,  Edge::Falling), 
        (100,  Edge::Rising), 
        (500,  Edge::Falling), 
        (100,  Edge::Rising), 
        (1100, Edge::Falling), 
        (100,  Edge::Rising), 
        (1100, Edge::Falling), 
        (100,  Edge::Rising), 
        (500,  Edge::Falling),
        (100,  Edge::Rising), 
        (500,  Edge::Falling), 
        (100,  Edge::Rising), 
        (200,  Edge::Falling), 
        (100,  Edge::Rising), 
        (511,  Edge::Rising),
    ],
}];
