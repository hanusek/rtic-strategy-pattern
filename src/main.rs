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

    // Option 2

    // 1. Frame -> operate ( DO -> set outputs, DI -> set parameters)
    // 2. Read state -> Frame (Do -> read outputs, DI -> read inputs)

    fn do_process_frame(t: &mut DiDo, frame: u32) {
        //TODO: decode frame and operate on Do card (set outputs)
    }

    fn di_process_frame(t: &mut DiDo, frame: u32) {
        //TODO: decode frame and operate on Di card (e.g. set parameters of filter)
    }
    
    fn do_read_state(t: &mut DiDo) -> u32 {
        0
    }

    fn di_read_state(t: &mut DiDo) -> u32 {
        0
    }

    pub struct DiDo
    {
        inputs : Vec<ErasedPin<stm32f1xx_hal::gpio::Input>, 16>,
        outputs : Vec<ErasedPin<stm32f1xx_hal::gpio::Output>, 16>,
        read_state_fn: fn(t: &mut DiDo) -> u32,
        process_frame: fn(t: &mut DiDo, frame: u32)
    }

    impl DiDo
    {
        pub fn new_from(state: bool) -> Self {
            if state {
                Self{ 
                    inputs: Vec::new(), 
                    outputs: Vec::new(),
                    read_state_fn: do_read_state,
                    process_frame: do_process_frame
                }
            }
            else {
                Self{ 
                    inputs: Vec::new(), 
                    outputs: Vec::new(),
                    read_state_fn: di_read_state,
                    process_frame: di_process_frame
                }
            }
        }
    }

    #[shared]
    struct Shared {
        dido: DiDo,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) 
    {        
        let mut gpioa = cx.device.GPIOA.split();

        // DI or DO
        let dev_type_selector = gpioa.pa0.into_pull_down_input(&mut gpioa.crl);

        let dido = DiDo::new_from(dev_type_selector.is_high());
        (Shared {dido}, Local {}, init::Monotonics())
    }
}