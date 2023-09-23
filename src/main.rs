#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(not(test))]
use defmt_rtt as _;

#[cfg(not(test))]
use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    use heapless::Vec;
    use stm32f1xx_hal::prelude::*;
    use stm32f1xx_hal::gpio::ErasedPin;    

    // Option 1
    pub trait OperationStrategy
    {
        fn process_frame(&mut self, f: u32);
        fn read_state(&mut self) -> u32;
    }

    pub struct Di {
        inputs: Vec<ErasedPin<stm32f1xx_hal::gpio::Input>, 16>,
    }

    impl Di 
    {
        pub fn new() -> Self
        {
            Self{inputs: Vec::new()}
        }
    }

    pub struct Do {
        outputs: Vec<ErasedPin<stm32f1xx_hal::gpio::Output>, 16>
    }

    impl Do
    {
        pub fn new() -> Self
        {
            Self{outputs: Vec::new()}
        }
    }

    impl OperationStrategy for Di {
        fn process_frame(&mut self, f: u32) 
        {
            defmt::println!("Di - process_frame");
        }

        fn read_state(&mut self) -> u32
        {
            defmt::println!("Di - read inputs");
            0
        }
    }

    impl OperationStrategy for Do {
        fn process_frame(&mut self, f: u32) 
        {
            defmt::println!("Do - set outputs");
        }

        fn read_state(&mut self) -> u32
        {
            defmt::println!("Do - read outputs");
            0
        }
    }

    pub enum DevType {
        Di(Di),
        Do(Do),
    }

    impl DevType
    {
        pub fn new_do() -> Self
        {
            DevType::Do(Do::new())
        }

        pub fn new_di() -> Self
        {
            DevType::Di(Di::new())
        }

        fn process_frame(&mut self, f: u32) 
        {
            match self {
                DevType::Di(d) => { d.process_frame(f); },
                DevType::Do(d) => { d.process_frame(f); }
            };
        }

        fn read_state(&mut self) -> u32 
        {
            match self {
                DevType::Di(d) => { d.read_state() },
                DevType::Do(d) => { d.read_state() }
            }
        }
    }

    
    #[shared]
    struct Shared {
        dev_type: DevType,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) 
    {        
        let mut gpioa = cx.device.GPIOA.split();

        // DI or DO
        let dev_type_selector = gpioa.pa0.into_pull_down_input(&mut gpioa.crl);

        let dev_type = if dev_type_selector.is_high() { DevType::new_di() } else {  DevType::new_do() };
        (Shared {dev_type}, Local {}, init::Monotonics())
    }
}