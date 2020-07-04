use super::crkcam::cmn::Edge;
use super::crkcam::{self, cam::*, crk::*};
use super::periph;

const CRK_CAM_AUTORELOAD: u32 = 36_000;
const TIM_MIN_FROM_S: u32 = 60;

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
    );
}

fn init_timer(tim: &stm32f1::stm32f103::tim2::RegisterBlock) {
    tim.cr1.modify(|_, w| {
        w.ckd().div1()
         .arpe().disabled()
         .cms().edge_aligned()
         .dir().up()
         .opm().disabled()
         .urs().counter_only()
         .udis().enabled()
    });
}

fn init_gpio() {
    //A0 -> TIM2_CH1 and A6 -> TIM3_CH1, no remap needed
    let rcc = periph!(RCC);
    let pa = periph!(GPIOA);

    rcc.apb2enr.modify(|_, w| 
        w.iopaen().enabled()
         .iopben().enabled()
    );

    pa.crl.modify(|_, w| 
        w.cnf0().push_pull()
         .mode0().output2()
         .cnf6().push_pull()
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
            nvic.set_priority(interrupt::TIM3, 3);
            nvic.set_priority(interrupt::TIM2, 2);

            cortex_m::peripheral::NVIC::unmask(interrupt::TIM2);
            cortex_m::peripheral::NVIC::unmask(interrupt::TIM3);
        }

    }

    fn set_speed_rpm(&mut self, spd: u32) {
        let tim_crk = periph!(TIM2);
        let tim_cam = periph!(TIM3);

        self.speed = if spd > 0 { spd } else { 1 };
        let psc = (TIM_MIN_FROM_S * 2 * (self.freq/CRK_CAM_AUTORELOAD) / self.speed) - 1;
        self.prescaler = if psc > 0xFFFF {
            0xFFFF as u16
        } else {
            psc as u16
        };
        set_tim_psc(tim_crk, self.prescaler);
        set_tim_psc(tim_cam, self.prescaler);
    }

    fn set_next_crk_ev(&mut self) {
        let tim_crk = periph!(TIM2);
        let pa = periph!(GPIOA);
        let ev_ag = self.crk.as_mut().unwrap().next().unwrap();
        self.crk_arr = ev_ag.ag as u16;
        // Clear interrupt flags
        clr_ti_irq_flg(tim_crk);
        // Disable timer counter
        tim_crk.cr1.modify(|_, w| w.cen().disabled());
        // Load next event
        tim_crk.arr.write(|w| w.arr().bits(self.crk_arr));
        tim_crk.cnt.write(|w| w.cnt().bits(0));
        // Enable timer counter
        tim_crk.cr1.modify(|_, w| w.cen().enabled());

        if ev_ag.is_gen {
            match ev_ag.edge {
                Edge::Rising => pa.bsrr.write(|w| w.bs6().set()),
                Edge::Falling => pa.bsrr.write(|w| w.br6().reset()),
            }
        }
    }

    fn set_next_cam_ev(&mut self) {
        let tim_cam = periph!(TIM3);
        let tim_crk = periph!(TIM2);
        let pa = periph!(GPIOA);
        let ev_ag = self.cam.as_mut().unwrap().next().unwrap();
        self.cam_arr = ev_ag.ag as u16;
        // Disable timer counter
        tim_cam.cr1.modify(|_, w| w.cen().disabled());
        if ev_ag.id == 0 {
            tim_crk.cnt.write(|w| w.cnt().bits(0));
            self.crk.as_mut().unwrap().reset();
        }
        
        // Clear interrupt flags
        clr_ti_irq_flg(tim_cam);
        
        // Load next event, clear timer
        tim_cam.arr.write(|w| w.arr().bits(self.cam_arr));
        tim_cam.cnt.write(|w| w.cnt().bits(0));
        // Enable timer counter
        tim_cam.cr1.modify(|_, w| w.cen().enabled());

        match ev_ag.edge {
            Edge::Rising => pa.bsrr.write(|w| w.bs0().set()),
            Edge::Falling => pa.bsrr.write(|w| w.br0().reset()),
        }

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
