
pub fn init_clks() {
    unsafe {
        let dp = stm32f1::stm32f103::Peripherals::steal();
        let rcc = &dp.RCC;
        let fls = &dp.FLASH;
        // enable HSE clock
        rcc.cr.modify(|_, w| w.hseon().on());
        while rcc.cr.read().hserdy().is_not_ready() {}

        // configure wait state for flash with two wait states
        fls.acr.modify(|_, w| w.prftbe().set_bit());
        fls.acr.modify(|_, w| w.latency().ws2());

        // configure AHB speed to system clock
        // configure APB2 speed to system clock
        // configure APB1 speed to half the system clock
        rcc.cfgr.modify(|_, w| 
            w.hpre().div1()
             .ppre2().div1()
             .ppre1().div2()
        );

        // configure pll
        // The external crystal is a 8MHz one. 
        // To obtain 72MHz, the max, multiply by 9.
        // Multiply by 9
        rcc.cfgr.modify(|_, w| w.pllmul().bits(7));
        // Disable the hse-2-pll clock dividing
        rcc.cfgr.modify(|_, w| w.pllxtpre().div1());
        // Set HSE as the source of PLL
        rcc.cfgr.modify(|_, w| w.pllsrc().hse_div_prediv());
        // switch on pll
        rcc.cr.modify(|_, w| w.pllon().on());
        while rcc.cr.read().pllrdy().is_not_ready() {}

        // switch on the pll as system clock
        rcc.cfgr.modify(|_, w| w.sw().pll());
        while !rcc.cfgr.read().sws().is_pll() {}
    }
}
