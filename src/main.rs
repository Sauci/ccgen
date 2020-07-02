#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f1::stm32f103::interrupt;

mod crkcam;
mod hwsiggen;
mod system;
mod periph;
mod pot;


use crkcam::{cam::*, cam_cfg::*, crk::*, crk_cfg::*};
use crkcam::siggen::CrkCamSigGen;
use hwsiggen::Timer;

static mut GEN_TIM: Timer = Timer::new(36_000_000);

#[interrupt]
fn TIM2() {
    let tim = unsafe { &mut GEN_TIM };
    tim.set_next_crk_ev();
}

#[interrupt]
fn TIM3() {
    let tim = unsafe { &mut GEN_TIM };
    tim.set_next_cam_ev();
}

#[entry]
fn main() -> ! {
    system::init_clks();
    pot::init();
    
    let tim = unsafe { &mut GEN_TIM };
    {
        let mut crk_gen: CrkSigGen = CrkSigGen::new();
        let mut cam_gen: CamSigGen = CamSigGen::new();
        crk_gen.set_crk(&CRK_CONFIGS[0]);
        cam_gen.set_cam(&CAM_CONFIGS[0]);
        
        tim.initialize(cam_gen, crk_gen);
        tim.set_speed_rpm(1500);
        tim.start();
    }

    loop {
        let val = pot::get_val();
        tim.set_speed_rpm(val / 2);
        cortex_m::asm::delay(1_000_000);
    }
}
