#![no_std]
use core::panic::PanicInfo;
mod board;
mod hal;


#[no_mangle]
pub extern fn system_init() {
    /* RCC Enabling of the bus */
    let rcc = hal::rcc::Rcc::init(board::l432kc::RCC_BASE);

    rcc.write_msi_range(hal::rcc::MsiRange::Clk16MHz);
    rcc.write_ahb2_enr(board::l432kc::GPIOA_RCC_AHB2_ENABLE);
    rcc.write_ahb2_enr(board::l432kc::GPIOB_RCC_AHB2_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::TIMER2_RCC_APB1R1_ENABLE);
    rcc.write_apb1_enr1(board::l432kc::USART2_RCC_APB1R1_ENABLE);
}

#[no_mangle]
pub extern fn start() {
    let freq = hal::rcc::range(hal::rcc::MsiRange::Clk16MHz);
    // Initialize the LED on L432KC board
    let gpioa = hal::gpio::Gpio::init(board::l432kc::GPIOA_BASE);  
    let gpiob = hal::gpio::Gpio::init(board::l432kc::GPIOB_BASE);
    let usart = hal::usart::Usart::init(board::l432kc::USART2_BASE);
    let seq_timer = hal::timer::Timer::init(board::l432kc::TIMER2_BASE);

    /* USART Setup */
    gpioa.otype(board::l432kc::USART2_TX, hal::gpio::Mode::Alt, hal::gpio::OType::PushPull, hal::gpio::AltFunc::Af7);
    gpioa.otype(board::l432kc::USART2_RX, hal::gpio::Mode::Alt, hal::gpio::OType::PushPull, hal::gpio::AltFunc::Af7);

    gpiob.otype(board::l432kc::USER_LED, hal::gpio::Mode::Out, hal::gpio::OType::PushPull, hal::gpio::AltFunc::Af0);
    seq_timer.open(hal::timer::TimerType::Cont, hal::timer::Direction::Upcount);
    seq_timer.set_time(500, freq, 1500);
    seq_timer.start();

    usart.open(hal::usart::WordLen::Bits8, hal::usart::StopLen::StopBit1, hal::usart::BaudRate::Baud9600, freq, hal::usart::OverSample::Oversample16);

    let mut i = false;

    let dogmeat = [0x44, 0x6F, 0x67, 0x6D, 0x65, 0x61, 0x74, 0x0D];

    loop {
        if seq_timer.get_flag() {
            if i {
                gpiob.set_pin(board::l432kc::USER_LED_BIT);
                i = false;
            } else {
                gpiob.clr_pin(board::l432kc::USER_LED_BIT);
                i = true;
            }
            usart.write(&dogmeat);
            seq_timer.clr_flag();
        }
	}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
