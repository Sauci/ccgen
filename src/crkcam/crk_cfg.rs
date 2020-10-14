use super::cmn::*;
use super::crk::CrkCfg;

pub static CRK_CONFIGS: [CrkCfg; 6] = [
    CrkCfg::new(
        120, 
        2, 
        Edge::Falling
    ),
    CrkCfg::new(
        120, 
        1, 
        Edge::Falling
    ),
    CrkCfg::new(
        60, 
        2, 
        Edge::Falling
    ),
    CrkCfg::new(
        60, 
        1, 
        Edge::Falling
    ),
    CrkCfg::new(
        30, 
        2, 
        Edge::Falling
    ),
    CrkCfg::new(
        30, 
        1, 
        Edge::Falling
    )
];
