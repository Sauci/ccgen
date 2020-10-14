#![no_main]
#![no_std]

extern crate panic_halt;

mod crkcam;
mod hwsiggen;
mod system;
mod periph;
mod com;

use cortex_m_rt::entry;
use stm32f1::stm32f103::interrupt;
use cortex_m::asm;

use crkcam::{
    cam::*, 
    cam_cfg::*, 
    crk::*, 
    crk_cfg::*
};
use crkcam::siggen::CrkCamSigGen;
use hwsiggen::Timer;

static mut GEN_TIM: Timer = Timer::new(36_000_000);

#[interrupt]
fn TIM2() {
    unsafe { GEN_TIM.set_next_crk_ev() };
    unsafe { GEN_TIM.set_next_cam_ev() };
}

#[entry]
fn main() -> ! {
    system::init_clks();

    let mut speed = 1000;
    let mut cam_cfg_id = 0;
    let mut crk_cfg_id = 0;
    
    let tim = unsafe { &mut GEN_TIM };
    let crk_gen: CrkSigGen = CrkSigGen::new(&CRK_CONFIGS[crk_cfg_id]);
    let cam_gen: CamSigGen = CamSigGen::new(&CAM_CONFIGS[cam_cfg_id]);
    tim.initialize(cam_gen, crk_gen);
    tim.set_speed_rpm(speed);
    tim.start();
    
    loop { }
}
