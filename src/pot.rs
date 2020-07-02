use super::periph;

pub fn init() {
    let rcc = periph!(RCC);
    let pa = periph!(GPIOA);
    let adc = periph!(ADC1);

    rcc.apb2enr.modify(|_, w| 
        w.iopaen().enabled()
         .adc1en().enabled()
    );

    pa.crl.modify(|_, w|
        w.mode4().input()
         .cnf4().push_pull()
    );

    adc.cr2.modify(|_, w| w.align().right());

    adc.smpr2.modify(|_, w| w.smp2().cycles239_5());
    adc.sqr3.modify(|_, w| unsafe{ w.sq1().bits(4) });

    adc.cr2.modify(|_, w| w.adon().enabled());
}

pub fn get_val() -> u32 {
    let adc = periph!(ADC1);
    adc.cr2.modify(|_, w| w.adon().enabled());
    while adc.sr.read().eoc().is_not_complete() {}
    adc.dr.read().bits()
}