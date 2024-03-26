//#![feature(error_in_core)]
//#![feature(associated_type_defaults)]
//#![no_std]

use core::panic;

use embassy_executor::Executor;
use embassy_stm32::spi::{MisoPin, MosiPin, SckPin, TxDma, RxDma, Instance};
use embassy_stm32::{peripherals, Config};
use spi::spi::{ EmbassySpi, SpiInstance };
use spi::config::{SpiConfig, SpiConfigStruct};
use crate::baro::baro::Baro;

pub mod spi;
pub mod delay;
mod baro;

pub struct Peripherals {
    pub spi: SpiInstance<TempConfig>,
}

impl Peripherals {
    pub fn new(p: embassy_stm32::Peripherals) -> Self {
        Self {
            spi:SpiInstance::new(SpiConfigStruct {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PA7,
                dma_tx: p.DMA1_CH1,
                dma_rx: p.DMA1_CH2
            }),
        }
    }
}

pub struct Sirin {
    pub peripherals: Peripherals,
    pub executor: Executor,
}

impl Sirin {
    pub fn new() -> Self {
        Self::from_config(Config::default())
    }

    pub fn from_config(config: Config) -> Self {
        let embassy_peripherals = embassy_stm32::init(config);

        Self {
            peripherals: Peripherals::new(embassy_peripherals),
            executor: Executor::new(),
        }
    }
}

pub struct TempConfig {
}
impl SpiConfig for TempConfig {
    type Spi = embassy_stm32::peripherals::SPI1;
    type Sck = embassy_stm32::peripherals::PA5;
    type Miso = embassy_stm32::peripherals::PA6;
    type Mosi = embassy_stm32::peripherals::PA7;
    type RxDma = embassy_stm32::peripherals::DMA1_CH2;
    type TxDma = embassy_stm32::peripherals::DMA1_CH1;
}