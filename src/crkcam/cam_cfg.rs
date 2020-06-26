use super::cam::CamCfg;
use super::cmn::*;

pub const CAM_CONFIGS: [CamCfg; 1] = [CamCfg {
    ev_nr: 21,
    mai_edge: Edge::Rising,
    ev_ag: [
        2890, 
        1000, 
        8000, 
        1000, 
        2000, 
        1000, 
        5000, 
        1000, 
        5000, 
        1000, 
        11000, 
        1000, 
        11000, 
        1000, 
        5000,
        1000, 
        5000, 
        1000, 
        2000, 
        1000, 
        5110,
    ],
}];
