#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f1::stm32f103::interrupt;

mod crkcam;
mod hwsiggen;
mod system;
mod periph;

use crkcam::{cam::*, cam_cfg::*, crk::*, crk_cfg::*};
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
    
    {
        let tim = unsafe { &mut GEN_TIM };
        let crk_gen: CrkSigGen = CrkSigGen::new(&CRK_CONFIGS[0]);
        let cam_gen: CamSigGen = CamSigGen::new(&CAM_CONFIGS[0]);
        
        tim.initialize(cam_gen, crk_gen);
        tim.set_speed_rpm(3000);
        tim.start();
    }

    loop {}
}
