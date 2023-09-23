#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(not(test))]
use defmt_rtt as _;

#[cfg(not(test))]
use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    use stm32f1xx_hal::prelude::*;

    pub trait OperationStrategy: Sized + 'static
    {
        fn operate(&mut self);
    }

    struct Di{}
    impl OperationStrategy for Di{
        fn operate(&mut self)
        {
            defmt::println!("Di - operate on inputs (read inputs)");
        }
    }

    struct Do{}
    impl OperationStrategy for Do{
        fn operate(&mut self)
        {
            defmt::println!("Do - operate on outputs (write outputs)");
        }
    }

    #[shared]
    struct Shared {
        dev_type: dyn OperationStrategy //not working
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) 
    {        
        let mut gpioa = cx.device.GPIOA.split();

        // DI or DO
        let dev_type_selector = gpioa.pa0.into_pull_down_input(&mut gpioa.crl);

        let dev_type;
        if dev_type_selector.is_high() {
            dev_type = Di{};
        }
        else {
            dev_type = Do{};
        }
        
        (Shared {dev_type}, Local {}, init::Monotonics())
    }
}