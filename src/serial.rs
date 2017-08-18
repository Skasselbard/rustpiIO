use std::io;
use std::io::prelude::*;
use spidev::{SPI_MODE_0, Spidev, SpidevOptions, SpidevTransfer};
use globals::{SPI_PATH0, SPI_PATH1};

//TODO: write to stdout

pub enum Device {
    CE0 = 0,
    CE1 = 1,
}

/** 
 From https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md#driver  
 Possible Speeds:  
    125.0 MHz  
    62.5 MHz  
    31.2 MHz  
    15.6 MHz  
    7.8 MHz  
    3.9 MHz  
    1953 kHz  
    976 kHz  
    488 kHz  
    244 kHz  
    122 kHz  
    61 kHz  
    30.5 kHz  
    15.2 kHz  
    7629 Hz  
 */
 #[allow(non_camel_case_types)]
pub enum Speed {
    Mhz125_0,
    Mhz62_5,
    Mhz31_2,
    Mhz15_6,
    Mhz7_8,
    Mhz3_9,
    Khz1953,
    Khz976,
    Khz488,
    Khz244,
    Khz122,
    Khz61,
    Khz30_5,
    Khz15_2,
    Hz7629,
}


impl Speed{
    /// Converts the `Speed` variants into an integer representing the Hz value
    #[allow(non_snake_case, unused)]
    fn to_int(&self) -> u32{
        match self{
            Mhz125_0 => 125_000_001,
            Mhz62_5 => 62_500_001,
            Mhz31_2 => 31_200_001,
            Mhz15_6 => 15_600_001,
            Mhz7_8 => 7_800_001,
            Mhz3_9 => 3_900_001,
            Khz1953 => 1_935_001,
            Khz976 => 976_001,
            Khz488 => 488_001,
            Khz244 => 244_001,
            Khz122 => 122_001,
            Khz61 => 61_001,
            Khz30_5 => 30_501,
            Khz15_2 => 15_201,
            Hz7629 => 7_630,
        }
    }
}

pub fn create_spi(device: Device, speed: Speed) -> io::Result<Spidev> {
    let mut spi = match device {
        Device::CE0 => try!(Spidev::open(SPI_PATH0)),
        Device::CE1 => try!(Spidev::open(SPI_PATH1)),
    };
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(speed.to_int())
        .mode(SPI_MODE_0)
        .build();
    try!(spi.configure(&options));
    Ok(spi)
}

/// Perform full duplex operations using Ioctl
pub fn full_duplex(spi: &Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let tx_buf = [0x01, 0x05, 0x03];
    let mut rx_buf = [0; 3];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        try!(spi.transfer(&mut transfer));
    }
    println!("{:?}", rx_buf);
    Ok(())
}
