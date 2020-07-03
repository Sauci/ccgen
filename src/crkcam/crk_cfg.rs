use super::cmn::*;
use super::crk::CrkCfg;

pub static CRK_CONFIGS: [CrkCfg; 1] = [
    CrkCfg::new(
        120, 
        2, 
        Edge::Falling
    )
];
