use super::crkcam::cmn::{Edge, Event};
use super::crkcam::{self, cam::*, crk::*};
use super::periph;

const CRK_CAM_AUTORELOAD: u32 = 0xFFFF;
const NR_OF_DEG_TICKS: u32 = 3_600;
const TIM_MIN_FROM_S: u32 = 60;

use stm32f1::stm32f103::interrupt;

fn wrapping_add(cv: u32, a: u32, lim: u32) -> u32 {
    (cv + a) % lim
}

pub struct Timer {
    cam: Option<CamSigGen>,
    crk: Option<CrkSigGen>,
    prescaler: u32,
    cam_nxt_ev: u16,
    cam_ev: Event,
    crk_nxt_ev: u16,
    crk_ev: Event,
    ///Generation speed, RPM
    speed: u32,
    ///Timer clock frequency, Hz
    freq: u32,
}

impl Timer {
    pub const fn new(freq: u32) -> Timer {
        Timer {
            cam: None,
            crk: None,
            prescaler: 1,
            cam_nxt_ev: 0,
            cam_ev: Event::new(),
            crk_nxt_ev: 0,
            crk_ev: Event::new(),
            speed: 0,
            freq,
        }
    }
}

fn init_timer(tim: &stm32f1::stm32f103::tim2::RegisterBlock) {
    tim.cr1.modify(|_, w| {
        w.ckd().div1()
            .arpe().disabled()
            .cms().edge_aligned()
            .dir().up()
            .opm().disabled()
            .urs().any_event()
            .udis().enabled()
    });

    tim.ccmr1_output_mut().modify(|_, w| {
        w.cc1s().output()
        .cc2s().output()
        .oc1pe().disabled()
        .oc2pe().disabled()
    });

    tim.ccer.modify(|_, w| {
        w.cc1e().set_bit() // output capture enabled
        .cc2e().set_bit()
        .cc1p().clear_bit() // active high
        .cc2p().clear_bit()
    });

    tim.arr.write(|w| w.arr().bits(CRK_CAM_AUTORELOAD as u16));
    tim.dier.modify(
        |_, w| {
            w.cc1ie()
             .enabled() // enable interrupt on output compare channel 1
             .cc2ie()   // enable inpurrupt on output compare channel 2
             .enabled()
        }, 
    );

    let dbg = periph!(DBGMCU);
    dbg.cr.modify(|_, w| w.dbg_tim2_stop().set_bit());
}

fn init_gpio() {
    //A0 -> TIM2_CH1 and A1 -> TIM2_CH2, no remap needed
    let rcc = periph!(RCC);
    let pa = periph!(GPIOA);

    rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

    pa.crl.modify(|_, w| {
        w.cnf0()
            .alt_push_pull()
            .mode0()
            .output()
            .cnf1()
            .alt_push_pull()
            .mode1()
            .output()
    });
}

impl crkcam::siggen::CrkCamSigGen for Timer {
    fn initialize(&mut self, cam: CamSigGen, crk: CrkSigGen) {
        let rcc = periph!(RCC);
        let tim = periph!(TIM2);

        rcc.apb1enr.modify(|_, w| w.tim2en().enabled());

        init_timer(tim);
        init_gpio();

        self.cam = Some(cam);
        self.crk = Some(crk);

        //Init interrupts
        unsafe {
            let mut nvic = cortex_m::Peripherals::steal().NVIC;
            nvic.set_priority(interrupt::TIM2, 2);
            cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
        }
    }

    fn set_speed_rpm(&mut self, spd: u32) {
        let tim = periph!(TIM2);

        // Check speed to be at least 0 to avoid division by 0.
        self.speed = if spd > 0 { spd } else { 1 };

        // Compute prescaler
        self.prescaler = TIM_MIN_FROM_S * 2 * (self.freq / NR_OF_DEG_TICKS) / self.speed;
        self.prescaler = if self.prescaler > 0xFFFF {
            0xFFFF
        } else {
            self.prescaler
        };

        tim.psc.write(|w| w.psc().bits(self.prescaler as u16 - 1));
        tim.egr.write(|w| w.ug().set_bit());
        self.start();
    }

    fn set_next_crk_ev(&mut self) {
        let tim = periph!(TIM2);

        // Check if this is really an event on the cam channel and clear it
        // otherwise, return without doing anything
        if tim.sr.read().cc1if().bit_is_clear() {
            return;
        } else {
            // Get event from the cam list
            self.crk_ev = self.crk.as_mut().unwrap().next().unwrap();
            // Compute next event, addition of last event angle with current, wrapping around 360deg
            self.crk_nxt_ev = wrapping_add(self.crk_ev.ag, self.crk_nxt_ev as u32, CRK_CAM_AUTORELOAD) as u16;
            // Set the next event timing
            tim.ccr1.write(|w| w.ccr().bits(self.crk_nxt_ev));
            // Program next output state, to be set on event
            if self.crk_ev.is_gen {
                match self.crk_ev.edge {
                    Edge::Rising => tim
                        .ccmr1_output_mut()
                        .modify(|_, w| w.oc1m().active_on_match()),
                    Edge::Falling => tim
                        .ccmr1_output_mut()
                        .modify(|_, w| w.oc1m().inactive_on_match()),
                }
            } else {
                tim.ccmr1_output().modify(|_, w| w.oc1m().frozen());
            }

            tim.sr.modify(|_, w| w.cc1if().clear());
        }
    }

    fn set_next_cam_ev(&mut self) {
        let tim = periph!(TIM2);

        // Check if this is really an event on the cam channel and clear it
        // otherwise, return without doing anything
        if tim.sr.read().cc2if().bit_is_clear() {
            return;
        } else {
            // Get event from the cam list
            self.cam_ev = self.cam.as_mut().unwrap().next().unwrap();
            // Update structure for debug purpose
            self.cam_nxt_ev = wrapping_add(self.cam_ev.ag, self.cam_nxt_ev as u32, CRK_CAM_AUTORELOAD) as u16;
    
            // Set the next event timing
            tim.ccr2.write(|w| w.ccr().bits(self.cam_nxt_ev));
    
            // Program next output state, to be set on event
            match self.cam_ev.edge {
                Edge::Rising => tim
                    .ccmr1_output_mut()
                    .modify(|_, w| w.oc2m().active_on_match()),
                Edge::Falling => tim
                    .ccmr1_output_mut()
                    .modify(|_, w| w.oc2m().inactive_on_match()),
            }
            tim.sr.modify(|_, w| w.cc2if().clear());
        }
    }

    fn start(&mut self) {
        let tim = periph!(TIM2);
        tim.cnt.write(|w| unsafe{w.bits(0)});
        {
            // Get event from the crk list
            self.crk_ev = self.crk.as_mut().unwrap().next().unwrap();
            // Compute next event, addition of last event angle with current, wrapping around 360deg
            self.crk_nxt_ev = wrapping_add(self.crk_ev.ag, self.crk_nxt_ev as u32, CRK_CAM_AUTORELOAD) as u16;
            // Set the next event timing
            tim.ccr1.write(|w| w.ccr().bits(self.crk_nxt_ev));
            // Program next output state, to be set on event
            if self.crk_ev.is_gen {
                match self.crk_ev.edge {
                    Edge::Rising => tim
                        .ccmr1_output_mut()
                        .modify(|_, w| w.oc1m().active_on_match()),
                    Edge::Falling => tim
                        .ccmr1_output_mut()
                        .modify(|_, w| w.oc1m().inactive_on_match()),
                }
            } else {
                tim.ccmr1_output().modify(|_, w| w.oc1m().frozen());
            }
        }
        {
            // Get event from the cam list
            self.cam_ev = self.cam.as_mut().unwrap().next().unwrap();
            // Update structure for debug purpose
            self.cam_nxt_ev = wrapping_add(self.cam_ev.ag, self.cam_nxt_ev as u32, CRK_CAM_AUTORELOAD) as u16;
    
            // Set the next event timing
            tim.ccr2.write(|w| w.ccr().bits(self.cam_nxt_ev));
    
            // Program next output state, to be set on event
            match self.cam_ev.edge {
                Edge::Rising => tim
                    .ccmr1_output_mut()
                    .modify(|_, w| w.oc2m().active_on_match()),
                Edge::Falling => tim
                    .ccmr1_output_mut()
                    .modify(|_, w| w.oc2m().inactive_on_match()),
            }
        }
        tim.cr1.modify(|_, w| w.cen().enabled());
    }
}
