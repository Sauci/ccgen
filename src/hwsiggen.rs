use super::crkcam::cmn::Edge;
use super::crkcam::{self, cam::*, crk::*};
use super::periph;
use crate::crkcam::cmn::spd_ag_to_ti;

const CRK_CAM_AUTORELOAD: u16 = 36_000;

use stm32f1::stm32f103::interrupt;

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
    tim.sr.write(|w|
        w.uif().clear()
        .cc1if().clear()
        .cc2if().clear()
        .cc3if().clear()
        .cc4if().clear()
    );
}

fn init_timer(tim: &stm32f1::stm32f103::tim2::RegisterBlock) {
    tim.cr1.modify(|_, w| {
        w.ckd().div1()
         .arpe().enabled()
         .cms().edge_aligned()
         .dir().up()
         .opm().disabled()
         .urs().counter_only()
         .udis().enabled()
    });

    tim.ccer.modify(|_, w| w.cc1e().set_bit().cc1p().clear_bit());
}

fn init_gpio() {
    //A0 -> TIM2_CH1 and B0 -> TIM3_CH3, no remap needed
    let rcc = periph!(RCC);
    let pa = periph!(GPIOA);

    rcc.apb2enr.modify(|_, w| 
        w.iopaen().enabled()
         .iopben().enabled()
    );

    pa.crl.modify(|_, w| 
        w.cnf0().alt_push_pull()
         .mode0().output2()
         .cnf6().alt_push_pull()
         .mode6().output2()
    );
}

fn set_tim_psc(tim : &stm32f1::stm32f103::tim2::RegisterBlock, psc : u16) {
    tim.cr1.modify(|_, w| w.cen().disabled());
    tim.psc.write(|w| w.psc().bits(psc));
    tim.cr1.modify(|_, w| w.cen().enabled());
}

impl crkcam::siggen::CrkCamSigGen for Timer {
    fn initialize(&mut self, cam : CamSigGen, crk : CrkSigGen) {
        let rcc = periph!(RCC);
        let tim_crk = periph!(TIM2);
        let tim_cam = periph!(TIM3);

        rcc.apb1enr.modify(|_, w| 
            w.tim2en().enabled()
             .tim3en().enabled()
        );

        init_timer(tim_crk);
        init_timer(tim_cam);
        init_gpio();

        self.cam = Some(cam);
        self.crk = Some(crk);

        //Init interrupts
        unsafe {
            let mut nvic = cortex_m::Peripherals::steal().NVIC;
            nvic.set_priority(interrupt::TIM2, 6);
            nvic.set_priority(interrupt::TIM3, 5);

            cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
            cortex_m::peripheral::NVIC::unmask(interrupt::TIM3);
        }

    }

    fn set_speed_rpm(&mut self, spd: u32) {
        let tim_crk = periph!(TIM2);
        let tim_cam = periph!(TIM3);

        self.speed = spd;
        let psc =
            ((self.speed as u64) * (self.freq as u64) / (60 * CRK_CAM_AUTORELOAD as u64)) as u64;
        self.prescaler = if psc > 0xFFFF {
            0xFFFF as u16
        } else {
            self.prescaler as u16
        };
        set_tim_psc(tim_crk, self.prescaler);
        set_tim_psc(tim_cam, self.prescaler);
    }

    fn set_next_crk_ev(&mut self) {
        let tim_crk = periph!(TIM2);
        let ev_ag = self.crk.as_mut().unwrap().next().unwrap();
        self.crk_arr = spd_ag_to_ti(self.speed, ev_ag.ag);

        // Disable timer counter
        tim_crk.cr1.modify(|_, w| w.cen().disabled());

        // Clear interrupt flags
        clr_ti_irq_flg(tim_crk);

        // Load next event
        tim_crk.arr.write(|w| w.arr().bits(self.crk_arr));
        tim_crk.ccr1.write(|w| w.ccr().bits(self.crk_arr));

        if ev_ag.is_gen {
            match ev_ag.edge {
                Edge::Rising => tim_crk
                    .ccmr1_output_mut()
                    .modify(|_, w| w.oc1m().force_active()),
                Edge::Falling => tim_crk
                    .ccmr1_output_mut()
                    .modify(|_, w| w.oc1m().force_inactive()),
            }
        } else {
            tim_crk
                .ccmr1_output_mut()
                .modify(|_, w| w.oc1m().inactive_on_match());
        }

        // Enable timer counter
        tim_crk.cr1.modify(|_, w| w.cen().enabled());
    }

    fn set_next_cam_ev(&mut self) {
        let tim_cam = periph!(TIM3);
        let ev_ag = self.cam.as_mut().unwrap().next().unwrap();
        self.cam_arr = spd_ag_to_ti(self.speed, ev_ag.ag);

        // Disable timer counter
        tim_cam.cr1.modify(|_, w| w.cen().disabled());

        // Clear interrupt flags
        clr_ti_irq_flg(tim_cam);

        // Load next event
        tim_cam.arr.write(|w| w.arr().bits(self.cam_arr));
        tim_cam.ccr1.write(|w| w.ccr().bits(self.cam_arr));

        match ev_ag.edge {
            Edge::Rising => tim_cam
                .ccmr1_output_mut()
                .modify(|_, w| w.oc1m().force_active()),
            Edge::Falling => tim_cam
                .ccmr1_output_mut()
                .modify(|_, w| w.oc1m().force_inactive()),
        }

        // Enable timer counter
        tim_cam.cr1.modify(|_, w| w.cen().enabled());
    }

    fn start(&self) {
        let tim_crk = periph!(TIM2);
        let tim_cam = periph!(TIM3);

        // enable update event on timers
        tim_crk.dier.modify(|_, w| w.uie().enabled());
        tim_cam.dier.modify(|_, w| w.uie().enabled());

        // enable counter
        tim_crk.cr1.modify(|_, w| w.cen().enabled());
        tim_cam.cr1.modify(|_, w| w.cen().enabled());
    }
}
