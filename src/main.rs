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
    com::init();

    let speed = 1000;
    let cam_cfg_id = 0;
    let crk_cfg_id = 0;
    
    let tim = unsafe { &mut GEN_TIM };
    let crk_gen: CrkSigGen = CrkSigGen::new(&CRK_CONFIGS[crk_cfg_id]);
    let cam_gen: CamSigGen = CamSigGen::new(&CAM_CONFIGS[cam_cfg_id]);
    tim.initialize(cam_gen, crk_gen);
    tim.set_speed_rpm(speed);
    tim.start();
    
    let mut buf = [0; 32];
    loop {
        match com::read_data(&mut buf) {
            Ok(len) => {
                com::send_data(&buf[0..len]).unwrap();
            },
            Err(()) => (),
        }
        for _ in 0..10000 {
            cortex_m::asm::nop();
        }
    }
}
