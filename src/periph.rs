#[macro_export]
macro_rules! periph {
    ($periph:ident) => {{
        {
            unsafe { &stm32f1::stm32f103::Peripherals::steal().$periph }
        }
    }};
}
