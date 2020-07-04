use super::crkcam::cmn::Edge;
use super::crkcam::{self, cam::*, crk::*};
use super::periph;

const CRK_CAM_AUTORELOAD: u32 = 36_000;
const TIM_MIN_FROM_S: u32 = 60;

use stm32f1::stm32f103::interrupt;

fn wrapping_add(cv: u32, a: u32, lim: u32) -> u32 {
    (cv + a) % lim
}

pub struct Timer {
    cam: Option<CamSigGen>,
    crk: Option<CrkSigGen>,
    prescaler: u16,
    cam_arr: u16,
    crk_arr: u16,
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
            cam_arr: 0xFFFF,
            crk_arr: 0xFFFF,
            speed: 0,
            freq,
        }
    }
}

fn clr_ti_irq_flg(tim: &stm32f1::stm32f103::tim2::RegisterBlock) {
    tim.sr.write(|w| w.uif().clear());
}

fn init_timer(tim: &stm32f1::stm32f103::tim2::RegisterBlock) {
    tim.cr1.modify(|_, w| {
        w.ckd()
            .div1()
            .arpe()
            .disabled()
            .cms()
            .edge_aligned()
            .dir()
            .up()
            .opm()
            .disabled()
            .urs()
            .counter_only()
            .udis()
            .disabled()
    });

    tim.ccmr1_output_mut().modify(|_, w| {
        w.cc1s()
            .output()
            .cc2s()
            .output()
            .oc1pe()
            .disabled()
            .oc2pe()
            .disabled()
    });

    tim.ccer.modify(|_, w| {
        w.cc1e()
            .set_bit() // output capture enabled
            .cc2e()
            .set_bit()
            .cc1p()
            .clear_bit() // active high
            .cc2p()
            .clear_bit()
    });

    tim.arr.write(|w| w.arr().bits(CRK_CAM_AUTORELOAD as u16));
    tim.dier.modify(
        |_, w| {
            w.cc1ie()
                .enabled() // enable interrupt on output compare channel 1
                .cc2ie()
                .enabled()
        }, // enable inpurrupt on output compare channel 2
    );
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

fn set_tim_psc(tim: &stm32f1::stm32f103::tim2::RegisterBlock, psc: u16) {
    tim.cr1.modify(|_, w| w.cen().disabled());
    tim.psc.write(|w| w.psc().bits(psc));
    tim.cr1.modify(|_, w| w.cen().enabled());
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

        self.speed = if spd > 0 { spd } else { 1 };
        let psc = (TIM_MIN_FROM_S * 2 * (self.freq / CRK_CAM_AUTORELOAD) / self.speed) - 1;
        self.prescaler = if psc > 0xFFFF {
            0xFFFF as u16
        } else {
            psc as u16
        };
        set_tim_psc(tim, self.prescaler);
    }

    fn set_next_crk_ev(&mut self) {
        let tim = periph!(TIM2);

        // Check if this is really an event on the cam channel and clear it
        // otherwise, return without doing anything
        if tim.sr.read().cc1if().bit_is_clear() {
            return;
        } else {
            tim.sr.modify(|_, w| w.cc1if().clear());
        }

        // Get current timer counter value
        let cnt = tim.cnt.read().bits();

        // Get event from the cam list
        let ev_ag = self.crk.as_mut().unwrap().next().unwrap();

        // Update structure for debug purpose
        self.crk_arr = ev_ag.ag as u16;

        let next_ev = wrapping_add(cnt, self.crk_arr as u32, CRK_CAM_AUTORELOAD) as u16;

        // Set the next event timing
        tim.ccr1.write(|w| w.ccr().bits(next_ev));

        // Program next output state, to be set on event
        if ev_ag.is_gen {
            match ev_ag.edge {
                Edge::Rising => tim
                    .ccmr1_output()
                    .modify(|_, w| w.oc1m().active_on_match()),
                Edge::Falling => tim
                    .ccmr1_output()
                    .modify(|_, w| w.oc1m().inactive_on_match()),
            }
        } else {
            tim.ccmr1_output().modify(|_, w| w.oc1m().frozen());
        }
    }

    fn set_next_cam_ev(&mut self) {
        let tim = periph!(TIM2);

        // Check if this is really an event on the cam channel and clear it
        // otherwise, return without doing anything
        if tim.sr.read().cc2if().bit_is_clear() {
            return;
        } else {
            tim.sr.modify(|_, w| w.cc2if().clear());
        }
        // Get current timer counter value
        let cnt = tim.cnt.read().bits();

        // Get event from the cam list
        let ev_ag = self.cam.as_mut().unwrap().next().unwrap();

        // Update structure for debug purpose
        self.cam_arr = ev_ag.ag as u16;

        let next_ev = wrapping_add(cnt, self.cam_arr as u32, CRK_CAM_AUTORELOAD) as u16;

        // Set the next event timing
        tim.ccr2.write(|w| w.ccr().bits(next_ev));

        // if this is the cam event 0, reset on crank
        if ev_ag.id == 0 {
            tim.cnt.write(|w| w.cnt().bits(0));
            self.crk.as_mut().unwrap().reset();
        }

        // Program next output state, to be set on event
        match ev_ag.edge {
            Edge::Rising => tim
                .ccmr1_output()
                .modify(|_, w| w.oc2m().active_on_match()),
            Edge::Falling => tim
                .ccmr1_output()
                .modify(|_, w| w.oc2m().inactive_on_match()),
        }
    }

    fn start(&self) {
        let tim = periph!(TIM2);

        // enable update event on timers
        // enable counter
        tim.cr1.modify(|_, w| w.cen().enabled());
    }
}
