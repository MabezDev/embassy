#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::usart::Uart;
use embassy_time::Timer;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

embassy_stm32::bind_interrupts!(struct Irqs {
    USART2 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let uart = Uart::new_with_rtscts(
        p.USART2,
        p.PA3,
        p.PD5,
        Irqs,
        p.PD4,
        p.PA0,
        p.DMA1_CH6,
        p.DMA1_CH5,
        embassy_stm32::usart::Config::default(),
    )
    .expect("Failed to initialize uart 2");

    // This is board specific configuration to enable the port, you probably want to comment this out
    core::mem::forget(Output::new(p.PE7, Level::High, Speed::Low));
    core::mem::forget(Output::new(p.PD12, Level::Low, Speed::Low));
    core::mem::forget(Output::new(p.PB15, Level::Low, Speed::Low));
    core::mem::forget(Output::new(p.PD10, Level::High, Speed::Low));
    core::mem::forget(Output::new(p.PD11, Level::Low, Speed::Low));
    core::mem::forget(Output::new(p.PD13, Level::Low, Speed::Low));
    core::mem::forget(Output::new(p.PD14, Level::Low, Speed::Low));
    core::mem::forget(Output::new(p.PD15, Level::Low, Speed::Low));

    let (tx, rx) = uart.split();

    static BUFFER: StaticCell<[u8; 2048]> = StaticCell::new();
    let mut rx = rx.into_ring_buffered(BUFFER.init([0u8; 2048]));
    // uncomment this to witness breakage
    // rx.set_config(&Default::default()).unwrap();
    let mut buf = [0u8; 4096];
    loop {
        let len = rx.read(&mut buf[..]).await.unwrap();
        info!("Read {}bytes", len);
    }
}
